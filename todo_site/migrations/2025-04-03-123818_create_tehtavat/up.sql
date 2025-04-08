-- Your SQL goes here
CREATE TABLE tehtavat (
    id SERIAL PRIMARY KEY,
    paivamaara DATE NOT NULL,
    tyoaika FLOAT8 NOT NULL,
    lyhyt_kuvaus TEXT NOT NULL,
    tarkempi_kuvaus TEXT NOT NULL
);
