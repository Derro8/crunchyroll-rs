use crate::utils::Store;
use crate::utils::SESSION;
use crunchyroll_rs::Movie;

mod utils;

static MOVIE: Store<Movie> = Store::new(|| {
    Box::pin(async {
        let crunchy = SESSION.get().await?;
        let movie = crunchy.media_from_id("GM8F313NQ").await?;
        Ok(movie)
    })
});

#[tokio::test]
async fn movie_from_id() {
    assert_result!(MOVIE.get().await)
}

#[tokio::test]
async fn movie_stream() {
    assert_result!(MOVIE.get().await.unwrap().stream().await)
}

#[tokio::test]
async fn movie_alternative_stream() {
    assert_result!(MOVIE.get().await.unwrap().alternative_stream().await)
}
