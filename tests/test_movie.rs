use crate::utils::Store;
use crate::utils::SESSION;
use crunchyroll_rs::common::FromId;
use crunchyroll_rs::media::{Playback, Streams};
use crunchyroll_rs::Movie;

mod utils;

static MOVIE: Store<Movie> = Store::new(|| {
    Box::pin(async {
        let crunchy = SESSION.get().await?;
        let movie = Movie::from_id(crunchy, "G25FVGDEK".to_string()).await?;
        Ok(movie)
    })
});

#[tokio::test]
async fn movie_from_id() {
    assert_result!(MOVIE.get().await)
}

#[tokio::test]
async fn movie_playback() {
    let movie = MOVIE.get().await.unwrap();

    assert_result!(movie.playback().await)
}

#[tokio::test]
async fn movie_streams() {
    let movie = MOVIE.get().await.unwrap();

    assert_result!(movie.streams().await)
}
