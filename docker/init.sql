CREATE TABLE tyotiedot (
    id SERIAL PRIMARY KEY,
    paivamaara DATE NOT NULL,
    tyoaika NUMERIC NOT NULL,
    lyhyt_kuvaus VARCHAR(255),
    tarkempi_kuvaus TEXT
);
