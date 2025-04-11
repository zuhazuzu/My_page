# Tässä on tuntikirjaus verkkosivun suunnittelua varten.

| Päivä         | Käytetty aika(h) |  Tehdyn työn kuvaus        |
| :---          | :-----------:    | :----------------:         |
| 18.3.2025     |  1.5             | ideointi                   |
| 23.3.2025     |  3               | Määrittely                 |
| 31.3.2025     |  2.5             | Rust depencyjen asennus    |
| 3.4.2025      |  2               | postgreSQL asennus         |
| 8.4.2025      |  4               | tietokannan asennusta ja kontin tekoa |
| 9.4.2025      |  3               | Uudelleen aloitus ohjeen mukaisesti |
| 10.4.2025     |  3               | Rust kielen analysointia ja muistelua |
| 11.4.2025     |  3               | tietokannan "etsimistä" |
| 11.4.2025     |  3               | Frontti läjään |


- Kontin asenuksen kanssa ei mennyt ihan kuin tanssi. mutta luulen että se on ihan "ok" nytte. -8.4.2025 klo14.06
- Rustia aiemmin(3.4.2025) touhasin ja nyt se on ihan solmussa, koitan oikaista -8.4.2025 klo14.38
    - Tässä luovuin projektin nykyisestä rakenteesta, poistin sen ja päätin aloittaa alusta. ongelmat liittyi.

- backend paskaa(Jos luet tätä, olen unohtanut kirjoittaa siitä)
- 11.4.2025: Ongelma: Dockerkontin pystyttäminen onnistuu ja backendin pystyttäminen onnistuu, mutta backend ei löydä Dockerissa pyörivää tietokantaa. Backend kuitenkin ilmoittaa konsolissa "Table created successfully" ja pystyn tekemään erilaisia CRUD toimintoja, johonkin sen tiedon on mentävä. Selvittäessä asiaa ilmenee, että Docker kontti ei onnistuneesta ilmoituksesta huolimatta ole pystyssä. Mihin tieto sitten menee?: Ohjelmassa käytettävä tokio_postgres importilla on sellainen ominaisuus, että jos tämä ei löydä tietokantaa johon sen pitäisi yhdistää, niin se sellaisen luo. ELI: tietokanta on olemassa ja annetut tiedot menee jonnekkin. MUTTA, en ole selvittänyt miksi tokio_postgres toimii portissa 5432 ja Docker-tietokanta nousee samaan porttiin? spekulaatio on että tuo Docker kontti ei oikeasti nouse mihinkään.
- 11.4.2025: Tässä vaiheessa unohdetaan edellämainittu ongelma ja ryhdytään toimiin tiedon käsittelystä sivustolla. Käytetään Frontedissä apuna YEW-frameworkkia, jotta voidaan luoda frontend Rustilla(Rustin versio REACT:ista, käyttää WebAssemblyä apunaan) asennetaan riippuvuudet. Lisäksi Käytetään TRUNK rakennustyökalua joka auttaa pystyttämään YEW-framen(Kääntää Rustin WebAssemblyksi ja yhdistää assetit). Ohjelmassa käytetään myös *tailWind CSS* frameworkkia jotta saadaan ulkonäköön liittyvät toiminnot pelaamaan(Lisätään CSS suoraan HTML-koodiin tai komponentteihin). 
