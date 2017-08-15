extern crate csv;

use std::prelude::*;
use std::error::Error;
use std::fs::File;
use std::collections::HashMap;

#[derive(Debug)]
struct Movie {
    id: u32,
    title: String,
    genres: String
}

impl Movie {
    fn genres(&self) -> Vec<&str> {
        self.genres.split("|").collect()
    }
}

#[derive(Debug)]
struct Rating {
    user_id: u32,
    movie_id: u32,
    rating: f32
}

fn parse_ratings() -> Result<Vec<Rating>, Box<Error>> {
    let file = File::open("data/ml-latest-small/ratings.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    let ratings = rdr.records()
        .map(|result| {
            let record = result.unwrap();
            Rating {
                user_id: (&record[0]).parse().unwrap(),
                movie_id: (&record[1]).parse().unwrap(),
                rating: (&record[2]).parse().unwrap()
            }
        })
        .collect();

    Ok(ratings)
}

fn parse_movies() -> Result<Vec<Movie>, Box<Error>> {
    let file = File::open("data/ml-latest-small/movies.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    let v = rdr.records()
        .map(|result| {
            let record = result.unwrap();
            let id: u32 = record[0].parse().unwrap();
            let title = String::from(&record[1]);
            let genres = String::from(&record[2]);

            Movie {
                id: id,
                title: title,
                genres: genres
            }
        })
        .collect();

    Ok(v)
}

fn similarity(movie_rating: &Vec<&Rating>, movie_rating2: &Vec<&Rating>) -> f32 {
    let mut v: Vec<(&Rating, &Rating)> = vec![];
    for r1 in movie_rating {
        for r2 in movie_rating2 {
            if r1.user_id == r2.user_id {
                v.push((r1, r2))
            }
        }
    };

    let s1: f32 = v.iter().map(|r| {
        r.0.rating
    }).sum();

    let s2: f32 = v.iter().map(|r| {
        r.1.rating
    }).sum();

    let cos: f32 = v.into_iter().map(|(r1,r2)| {
        r1.rating * r2.rating
    }).sum();

    cos / (s1.sqrt() * s2.sqrt())
}

fn find_movie<'a>(movies: &'a Vec<Movie>, movie_id: u32) -> &'a Movie {
    movies.iter().find(|m| m.id == movie_id).unwrap()
}

fn run() -> Result<(), Box<Error>> {
    let movies = parse_movies()?;

    let ratings = parse_ratings()?;
    let mut movie_ratings = HashMap::new();

    for rating in &ratings {
        let mut movie_rating = movie_ratings.entry(rating.movie_id).or_insert(vec![]);
        movie_rating.push(rating);
    };

    let movie_ratings = movie_ratings;

    for (k1, r1) in &movie_ratings {
        let mut similarities = vec![];
        for (k2, r2) in &movie_ratings {
            let s = similarity(r1, r2);
            if !s.is_nan() {
                similarities.push((k2, s));
            }
        }

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similarities.truncate(10);

        println!("{:?}", find_movie(&movies, *k1).title);
        for (k2, _) in similarities {
            println!("{:?}", find_movie(&movies, *k2).title)
        }
        println!("\n")
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("{:?}", e)
    }
}
