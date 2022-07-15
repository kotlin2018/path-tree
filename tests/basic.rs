use path_tree::PathTree;
use rand::seq::SliceRandom;

#[test]
fn new_tree() {
    let mut tree: PathTree<usize> = PathTree::default();

    const ROUTES: [&str; 14] = [
        "",
        "/",
        "/users",
        "/users/:id",
        "/users/:id/:org",
        "/users/:user_id/repos",
        "/users/:user_id/repos/:id",
        "/users/:user_id/repos/:id/*any",
        "/:username",
        "/*any",
        "/about",
        "/about/",
        "/about/us",
        "/users/repos/*any",
    ];

    const VALID_URLS: [&str; 14] = [
        "",
        "/",
        "/users",
        "/users/fundon",
        "/users/fundon/viz-rs",
        "/users/fundon/repos",
        "/users/fundon/repos/path-tree",
        "/users/fundon/repos/viz-rs/viz",
        "/fundon",
        "/fundon/viz-rs/viz",
        "/about",
        "/about/",
        "/about/us",
        "/users/repos/viz-rs/viz",
    ];

    let valid_res = vec![
        vec![],
        vec![],
        vec![],
        vec![("id", "fundon")],
        vec![("id", "fundon"), ("org", "viz-rs")],
        vec![("user_id", "fundon")],
        vec![("user_id", "fundon"), ("id", "path-tree")],
        vec![("user_id", "fundon"), ("id", "viz-rs"), ("any", "viz")],
        vec![("username", "fundon")],
        vec![("any", "fundon/viz-rs/viz")],
        vec![],
        vec![],
        vec![],
        vec![("any", "viz-rs/viz")],
    ];

    let mut routes = ROUTES
        .iter()
        .zip(VALID_URLS.iter())
        .zip(valid_res.into_iter())
        .map(|(a, b)| (*a.0, *a.1, b))
        .collect::<Vec<_>>();

    routes.shuffle(&mut rand::thread_rng());

    for (i, (u, ..)) in routes.iter().enumerate() {
        tree.insert(u, i);
    }

    for (i, (_, u, v)) in routes.iter().enumerate() {
        let res = tree.find(u).unwrap();
        // println!("{}, {}, {:#?}", u, i, res);
        assert_eq!(*res.0, i);
        assert_eq!(res.1, *v);
    }
}

#[test]
fn statics() {
    let mut tree = PathTree::<usize>::new();

    const ROUTES: [&str; 11] = [
        "/hi",
        "/contact",
        "/co",
        "/c",
        "/a",
        "/ab",
        "/doc/",
        "/doc/go_faq.html",
        "/doc/go1.html",
        "/α",
        "/β",
    ];

    let mut routes = ROUTES.to_vec();

    routes.shuffle(&mut rand::thread_rng());

    for (i, u) in routes.iter().enumerate() {
        tree.insert(u, i);
    }

    for (i, u) in routes.iter().enumerate() {
        let res = tree.find(u).unwrap();
        // println!("{}, {}, {:#?}", u, i, res);
        assert_eq!(*res.0, i);
    }
}

#[test]
fn wildcards() {
    let mut tree = PathTree::<usize>::new();

    const ROUTES: [&str; 20] = [
        "/",
        "/cmd/:tool/:sub",
        "/cmd/:tool/",
        "/cmd/vet",
        "/src/*filepath",
        "/src1/",
        "/src1/*filepath",
        "/src2*filepath",
        "/search/",
        "/search/:query",
        "/search/invalid",
        "/user_:name",
        "/user_:name/about",
        "/user_x",
        "/files/:dir/*filepath",
        "/doc/",
        "/doc/rust_faq.html",
        "/doc/rust1.html",
        "/info/:user/public",
        "/info/:user/project/:project",
    ];

    let mut routes = (0..20).zip(ROUTES.iter()).collect::<Vec<_>>();

    routes.shuffle(&mut rand::thread_rng());

    for (i, u) in routes.iter() {
        tree.insert(u, *i);
    }

    // println!("tree: {:#?}", tree);

    let valid_res = vec![
        ("/", 0, vec![]),
        ("/cmd/test/", 2, vec![("tool", "test")]),
        ("/cmd/test/3", 1, vec![("tool", "test"), ("sub", "3")]),
        ("/src/", 4, vec![]),
        ("/src/some/file.png", 4, vec![("filepath", "some/file.png")]),
        (
            "/search/someth!ng+in+ünìcodé",
            9,
            vec![("query", "someth!ng+in+ünìcodé")],
        ),
        ("/user_rust", 11, vec![("name", "rust")]),
        ("/user_rust/about", 12, vec![("name", "rust")]),
        (
            "/files/js/inc/framework.js",
            14,
            vec![("dir", "js"), ("filepath", "inc/framework.js")],
        ),
        ("/info/gordon/public", 18, vec![("user", "gordon")]),
        (
            "/info/gordon/project/rust",
            19,
            vec![("user", "gordon"), ("project", "rust")],
        ),
    ];

    for (u, h, p) in valid_res {
        let res = tree.find(u).unwrap();
        // println!("{}, {:#?}", u, res);
        assert_eq!(*res.0, h);
        assert_eq!(res.1, p);
    }
}

#[test]
fn single_named_parameter() {
    //  Pattern: /users/:id
    //
    //      /users/gordon              match
    //      /users/you                 match
    //      /users/gordon/profile      no match
    //      /users/                    no match
    let mut tree = PathTree::new();

    tree.insert("/users/:id", 0);

    let res = vec![
        ("/", false),
        ("/users/gordon", true),
        ("/users/you", true),
        ("/users/gordon/profile", false),
        ("/users/", false),
        ("/users", false),
    ];

    for (u, b) in res {
        let r = tree.find(u);
        assert_eq!(r.is_some(), b);
    }
}

#[test]
fn static_and_named_parameter() {
    //  Pattern: /a/b/c
    //  Pattern: /a/c/d
    //  Pattern: /a/c/a
    //  Pattern: /:id/c/e
    //
    //      /a/b/c                  match
    //      /a/c/d                  match
    //      /a/c/a                  match
    //      /a/c/e                  match
    let mut tree = PathTree::new();

    tree.insert("/a/b/c", "/a/b/c");
    tree.insert("/a/c/d", "/a/c/d");
    tree.insert("/a/c/a", "/a/c/a");
    tree.insert("/:id/c/e", "/:id/c/e");

    let res = vec![
        ("/", false, "", vec![]),
        ("/a/b/c", true, "/a/b/c", vec![]),
        ("/a/c/d", true, "/a/c/d", vec![]),
        ("/a/c/a", true, "/a/c/a", vec![]),
        ("/a/c/e", true, "/:id/c/e", vec![("id", "a")]),
    ];

    for (u, b, a, p) in res {
        let r = tree.find(u);
        assert_eq!(r.is_some(), b);
        if let Some(res) = r {
            assert_eq!(*res.0, a);
            assert_eq!(res.1, p);
        }
    }
}

#[test]
fn multi_named_parameters() {
    //  Pattern: /:lang/:keyword
    //  Pattern: /:id
    //
    //      /rust                     match
    //      /rust/let                 match
    //      /rust/let/const           no match
    //      /rust/let/                no match
    //      /rust/                    no match
    //      /                         no match
    let mut tree = PathTree::new();

    tree.insert("/:lang/:keyword", true);
    tree.insert("/:id", true);

    let res = vec![
        ("/", false, false, vec![]),
        ("/rust/", false, false, vec![]),
        ("/rust/let/", false, false, vec![]),
        ("/rust/let/const", false, false, vec![]),
        (
            "/rust/let",
            true,
            true,
            vec![("lang", "rust"), ("keyword", "let")],
        ),
        ("/rust", true, true, vec![("id", "rust")]),
    ];

    for (u, b, a, p) in res {
        let r = tree.find(u);
        assert_eq!(r.is_some(), b);
        if let Some(res) = r {
            assert_eq!(*res.0, a);
            assert_eq!(res.1, p);
        }
    }
}

#[test]
fn catch_all_parameter() {
    //  Pattern: /src/*filepath
    //
    //      /src                      no match
    //      /src/                     match
    //      /src/somefile.go          match
    //      /src/subdir/somefile.go   match
    let mut tree = PathTree::new();

    tree.insert("/src/*filepath", "* files");

    let res = vec![
        ("/src", false, vec![]),
        ("/src/", true, vec![]),
        ("/src/somefile.rs", true, vec![("filepath", "somefile.rs")]),
        (
            "/src/subdir/somefile.rs",
            true,
            vec![("filepath", "subdir/somefile.rs")],
        ),
        ("/src.rs", false, vec![]),
        ("/rust", false, vec![]),
    ];

    for (u, b, p) in res {
        let r = tree.find(u);
        assert_eq!(r.is_some(), b);
        if let Some(res) = r {
            assert_eq!(*res.0, "* files");
            assert_eq!(res.1, p);
        }
    }

    tree.insert("/src/", "dir");

    let r = tree.find("/src/");
    assert!(r.is_some());
    if let Some(res) = r {
        assert_eq!(*res.0, "dir");
        assert_eq!(res.1, vec![]);
    }
}

#[test]
fn catch_all_parameter_with_prefix() {
    //  Pattern: /commit_*sha
    //
    //      /commit                   no match
    //      /commit_                  match
    //      /commit_/                 match
    //      /commit_/foo              match
    //      /commit_123               match
    //      /commit_123/              match
    //      /commit_123/foo           match
    let mut tree = PathTree::new();

    tree.insert("/commit_*sha", "* sha");
    tree.insert("/commit/:sha", "hex");
    tree.insert("/commit/:sha0/compare/:sha1", "compare");
    tree.insert("/src/", "dir");

    let r = tree.find("/src/");
    assert!(r.is_some());
    if let Some(res) = r {
        assert_eq!(*res.0, "dir");
        assert_eq!(res.1, vec![]);
    }

    let r = tree.find("/commit/123");
    assert!(r.is_some());
    if let Some(res) = r {
        assert_eq!(*res.0, "hex");
        assert_eq!(res.1, vec![("sha", "123")]);
    }

    let r = tree.find("/commit/123/compare/321");
    assert!(r.is_some());
    if let Some(res) = r {
        assert_eq!(*res.0, "compare");
        assert_eq!(res.1, vec![("sha0", "123"), ("sha1", "321")]);
    }

    let res = vec![
        ("/commit", false, vec![]),
        ("/commit_", true, vec![]),
        ("/commit_/", true, vec![("sha", "/")]),
        ("/commit_/foo", true, vec![("sha", "/foo")]),
        ("/commit123", false, vec![]),
        ("/commit_123", true, vec![("sha", "123")]),
        ("/commit_123/", true, vec![("sha", "123/")]),
        ("/commit_123/foo", true, vec![("sha", "123/foo")]),
    ];

    for (u, b, p) in res {
        let r = tree.find(u);
        assert_eq!(r.is_some(), b);
        if let Some(res) = r {
            assert_eq!(*res.0, "* sha");
            assert_eq!(res.1, p);
        }
    }
}

#[test]
fn static_and_catch_all_parameter() {
    //  Pattern: /a/b/c
    //  Pattern: /a/c/d
    //  Pattern: /a/c/a
    //  Pattern: /a/*c
    //
    //      /a/b/c                  match
    //      /a/c/d                  match
    //      /a/c/a                  match
    //      /a/c/e                  match
    let mut tree = PathTree::new();

    tree.insert("/a/b/c", "/a/b/c");
    tree.insert("/a/c/d", "/a/c/d");
    tree.insert("/a/c/a", "/a/c/a");
    tree.insert("/a/*c", "/a/*c");

    let res = vec![
        ("/", false, "", vec![]),
        ("/a/b/c", true, "/a/b/c", vec![]),
        ("/a/c/d", true, "/a/c/d", vec![]),
        ("/a/c/a", true, "/a/c/a", vec![]),
        ("/a/c/e", true, "/a/*c", vec![("c", "c/e")]),
    ];

    for (u, b, a, p) in res {
        let r = tree.find(u);
        assert_eq!(r.is_some(), b);
        if let Some(res) = r {
            assert_eq!(*res.0, a);
            assert_eq!(res.1, p);
        }
    }
}

#[test]
fn root_catch_all_parameter() {
    //  Pattern: /
    //  Pattern: /*
    //  Pattern: /users/*
    //
    //      /                  match *
    //      /download          match *
    //      /users/fundon      match users *
    let mut tree = PathTree::<fn() -> usize>::new();

    tree.insert("/", || 1);
    tree.insert("/*", || 2);
    tree.insert("/users/*", || 3);

    let res = vec![
        ("/", true, 1, vec![]),
        ("/download", true, 2, vec![("", "download")]),
        ("/users/fundon", true, 3, vec![("", "fundon")]),
    ];

    for (u, b, a, p) in res {
        let r = tree.find(u);
        assert_eq!(r.is_some(), b);
        if let Some(res) = r {
            assert_eq!(res.0(), a);
            assert_eq!(res.1, p);
        }
    }
}

#[test]
fn root_catch_all_parameter_1() {
    //  Pattern: /*
    //
    //      /                  match *
    //      /download          match *
    //      /users/fundon      match *
    let mut tree = PathTree::<fn() -> usize>::new();

    tree.insert("/*", || 1);

    let res = vec![
        ("/", true, 1, vec![]),
        ("/download", true, 1, vec![("", "download")]),
        ("/users/fundon", true, 1, vec![("", "users/fundon")]),
    ];

    // println!("tree: {:#?}", tree);

    for (u, b, a, p) in res {
        let r = tree.find(u);
        //println!("route: {:#?}", r);
        assert_eq!(r.is_some(), b);
        if let Some(res) = r {
            assert_eq!(res.0(), a);
            assert_eq!(res.1, p);
        }
    }

    tree.insert("/", || 0);
    let r = tree.find("/");
    //println!("route: {:#?}", r);
    assert!(r.is_some());
    if let Some(res) = r {
        assert_eq!(res.0(), 0);
        assert_eq!(res.1, []);
    }
}

#[test]
fn test_readme_example() {
    let mut tree = PathTree::<usize>::new();

    tree.insert("/", 0);
    tree.insert("/users", 1);
    tree.insert("/users/:id", 2);
    tree.insert("/users/:id/:org", 3);
    tree.insert("/users/:user_id/repos", 4);
    tree.insert("/users/:user_id/repos/:id", 5);
    tree.insert("/users/:user_id/repos/:id/*any", 6);
    tree.insert("/:username", 7);
    tree.insert("/*any", 8);
    tree.insert("/about", 9);
    tree.insert("/about/", 10);
    tree.insert("/about/us", 11);
    tree.insert("/users/repos/*any", 12);

    // Matched "/"
    let node = tree.find("/");
    assert!(node.is_some());
    let res = node.unwrap();
    assert_eq!(*res.0, 0);
    assert_eq!(res.1, []); // Params

    // Matched "/:username"
    let node = tree.find("/username");
    assert!(node.is_some());
    let res = node.unwrap();
    assert_eq!(*res.0, 7);
    assert_eq!(res.1, [("username", "username")]); // Params

    // Matched "/*any"
    let node = tree.find("/user/s");
    let res = node.unwrap();
    assert_eq!(*res.0, 8);
    assert_eq!(res.1, [("any", "user/s")]);

    // Matched "/users/:id"
    let node = tree.find("/users/fundon");
    let res = node.unwrap();
    assert_eq!(*res.0, 2);
    assert_eq!(res.1, [("id", "fundon")]); // Params

    // Matched "/users/:user_id/repos/:id"
    let node = tree.find("/users/fundon/repos/viz-rs");
    let res = node.unwrap();
    assert_eq!(*res.0, 5);
    assert_eq!(res.1, [("user_id", "fundon"), ("id", "viz-rs")]); // Params

    // Matched "/users/:user_id/repos/:id/*any"
    let node = tree.find("/users/fundon/repos/viz-rs/noder/issues");
    let res = node.unwrap();
    assert_eq!(*res.0, 6);
    assert_eq!(
        res.1,
        [
            ("user_id", "fundon"),
            ("id", "viz-rs"),
            ("any", "noder/issues"),
        ]
    ); // Params

    // Matched "/users/repos/*any"
    let node = tree.find("/users/repos/");
    let res = node.unwrap();
    assert_eq!(*res.0, 12);
    assert_eq!(res.1, []);
}

#[test]
fn test_named_routes_with_non_ascii_paths() {
    let mut tree = PathTree::<usize>::new();
    tree.insert("/", 0);
    tree.insert("/*any", 1);
    tree.insert("/matchme/:slug/", 2);

    // ASCII only (single-byte characters)
    let node = tree.find("/matchme/abc-s-def/");
    assert!(node.is_some());
    let res = node.unwrap();
    assert_eq!(*res.0, 2);
    assert_eq!(res.1, [("slug", "abc-s-def")]);

    // with multibyte character
    let node = tree.find("/matchme/abc-ß-def/");
    assert!(node.is_some());
    let res = node.unwrap();
    assert_eq!(*res.0, 2);
    assert_eq!(res.1, [("slug", "abc-ß-def")]);

    // with emoji (fancy multibyte character)
    let node = tree.find("/matchme/abc-⭐-def/");
    assert!(node.is_some());
    let res = node.unwrap();
    assert_eq!(*res.0, 2);
    assert_eq!(res.1, [("slug", "abc-⭐-def")]);

    // with multibyte character right before the slash (char boundary check)
    let node = tree.find("/matchme/abc-def-ß/");
    assert!(node.is_some());
    let res = node.unwrap();
    assert_eq!(*res.0, 2);
    assert_eq!(res.1, [("slug", "abc-def-ß")]);
}

#[test]
fn test_named_wildcard_collide() {
    let mut tree = PathTree::<usize>::new();
    tree.insert("/git/:org/:repo", 1);
    tree.insert("/git/*any", 2);

    let node = tree.find("/git/rust-lang/rust");
    assert!(node.is_some());
    let res = node.unwrap();
    assert_eq!(*res.0, 1);
    assert_eq!(res.1, [("org", "rust-lang"), ("repo", "rust")]);

    let node = tree.find("/git/rust-lang");
    assert!(node.is_some());
    let res = node.unwrap();
    assert_eq!(*res.0, 2);
    assert_eq!(res.1, [("any", "rust-lang")]);
}
