pub mod endpoints {

    //All endpoints are defined here
    pub const HOMEPAGE: &str = "https://api.themoviedb.org/3/trending/movie/week";
    pub const TOPRATED: &str = "https://api.themoviedb.org/3/movie/top_rated";
    pub const POPULAR: &str = "https://api.themoviedb.org/3/movie/popular";
    pub const BASEURL: &str = "https://api.themoviedb.org/3/movie/";
    pub const SEARCHMOVIE: &str = "https://api.themoviedb.org/3/search/movie";

    //Series endpoints
    pub const SERIESHOMEPAGE: &str = "https://api.themoviedb.org/3/trending/tv/week";
    pub const SERIESTOPRATED: &str = "https://api.themoviedb.org/3/tv/top_rated";
    pub const SERIESBASEURL: &str = "https://api.themoviedb.org/3/tv/";
    pub const SERIESPOPULAR: &str = "https://api.themoviedb.org/3/tv/popular";
    pub const SEARCHSERIES: &str = "https://api.themoviedb.org/3/search/tv";
}

pub mod creds {

    //To get credentials
    // VISIT -> https://www.themoviedb.org/settings/api
    pub const TOKEN:&str="eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJjZGFmMWJmODgyZGFiZTE5ZTk3ZThkZmY3NGUwNmEyMiIsInN1YiI6IjY0YjIzYjRiMjNkMjc4MDEwNzMwMWY1ZCIsInNjb3BlcyI6WyJhcGlfcmVhZCJdLCJ2ZXJzaW9uIjoxfQ.T_Ok57GRJ1M3_CSXlrpaNx0izPf8T3fhaMYDOBp4eEk";
}
