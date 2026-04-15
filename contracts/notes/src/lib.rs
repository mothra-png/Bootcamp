#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec,
};

// Storage key (FIX: maksimal 9 karakter)
const MOVIE_DATA: Symbol = symbol_short!("MOVIE");

// Struktur Movie
#[contracttype]
#[derive(Clone, Debug)]
pub struct Movie {
    pub id: u64,
    pub title: String,
    pub watched: bool,
}

#[contract]
pub struct MovieContract;

#[contractimpl]
impl MovieContract {
    // Ambil semua movie
    pub fn get_movies(env: Env) -> Vec<Movie> {
        env.storage()
            .instance()
            .get(&MOVIE_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Tambah movie
    pub fn add_movie(env: Env, title: String) -> String {
        // validasi sederhana
        if title.len() == 0 {
            return String::from_str(&env, "Title tidak boleh kosong");
        }

        let mut movies: Vec<Movie> = env
            .storage()
            .instance()
            .get(&MOVIE_DATA)
            .unwrap_or(Vec::new(&env));

        let movie = Movie {
            id: env.prng().gen::<u64>(),
            title,
            watched: false,
        };

        movies.push_back(movie);
        env.storage().instance().set(&MOVIE_DATA, &movies);

        String::from_str(&env, "Movie added")
    }

    // Tandai sudah ditonton
    pub fn mark_watched(env: Env, id: u64) -> String {
        let mut movies: Vec<Movie> = env
            .storage()
            .instance()
            .get(&MOVIE_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..movies.len() {
            let mut movie = movies.get(i).unwrap();

            if movie.id == id {
                movie.watched = true;
                movies.set(i, movie);
                env.storage().instance().set(&MOVIE_DATA, &movies);

                return String::from_str(&env, "Movie marked as watched");
            }
        }

        String::from_str(&env, "Movie not found")
    }

    // Hapus movie
    pub fn delete_movie(env: Env, id: u64) -> String {
        let mut movies: Vec<Movie> = env
            .storage()
            .instance()
            .get(&MOVIE_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..movies.len() {
            if movies.get(i).unwrap().id == id {
                movies.remove(i);
                env.storage().instance().set(&MOVIE_DATA, &movies);

                return String::from_str(&env, "Movie deleted");
            }
        }

        String::from_str(&env, "Movie not found")
    }

    // BONUS: Ambil movie yang belum ditonton
    pub fn get_unwatched(env: Env) -> Vec<Movie> {
        let movies: Vec<Movie> = env
            .storage()
            .instance()
            .get(&MOVIE_DATA)
            .unwrap_or(Vec::new(&env));

        let mut result = Vec::new(&env);

        for i in 0..movies.len() {
            let movie = movies.get(i).unwrap();
            if !movie.watched {
                result.push_back(movie);
            }
        }

        result
    }
}