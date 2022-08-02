use async_requests::{many_requests, many_requests_surf, print_collection};

mod async_requests;

fn main() {
    let requests = vec![
        ("example.com".to_string(), 80, "/".to_string()),
        ("www.red-bean.com".to_string(), 80, "/".to_string()),
        ("en.wikipedia.org".to_string(), 80, "/".to_string()),
    ];
    let requests_surf = &[
        "http://example.com".to_string(),
        "https://www.red-bean.com".to_string(),
        "https://en.wikipedia.org".to_string(),
    ];
    let results = async_std::task::block_on(many_requests(requests));
    let results_surf = async_std::task::block_on(many_requests_surf(requests_surf));

    print_collection(results.into_iter());
    print_collection(results_surf.into_iter());
}
