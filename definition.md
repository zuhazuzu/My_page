****
Tässä dokumentissa määritellään sivuston ominaisuuksia ja toimintoja 

Projekti luodaan Rust ohjelmointikieltä käyttäen ja siihen liittyvät tarpeellisuudet ovat 
1. Rocket FrameWork
2. Fronted tulee olemaan Yew frameworkilla ja siellä on apuna Gloo kirjastopaketti.

Rust-ohjelmointikielen haastavuus ja siihen liittyvät uudet teknologiat tekee projektin valmistumisesta epävarmaa.

## 1. Käyttäjäpersoonat:

| Käyttäjä         | Persoona     | Tavoite                                                                          | Halutut toiminnot                            | Käyttäytyminen                                                 |
| :--------------- | :----------- | :------------------------------------------------------------------------------- | :------------------------------------------- | -------------------------------------------------------------- |
| Otto oppija      | Opiskelija   | Kirjata käyttäjälistaa ja tarkastella sen sisältöä.                              | Tarkasella syötettyjä tietoja.               | Lisää säännöllisesti tietoa ja muuttaa sitä usein              |
| Maria Makia      | Harrastelija | Sivun pitää olla intuitiivinen ja helppokäyttöinen                               | Sivun helppokäyttöisyys.                     | Satunnainen käyttäjä, joka listää paljon tietoa kerralla.      |

## 2. Käyttötapaukset.



| Tapaus                 | Kuvaus                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Esimerkki                                                                                                                |
| :--------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| Tiedon lisääminen.     | Käyttäjä avaa sivuston jossa on kaksi lomaketta joiden sisällä on väliaikaiset otsikot: "Name" ja "Email". näihin lomakkeisiin käyttäjä täyttää tarvittavat tiedot jonka jälkeen painaessaan "Create user" painiketta (joka on jo sivun avatessa näkyvissä) syötetyt tiedot lisätään tietokantaan POST api pyynnön avulla ja käyttäjä saa tiedon "user created"                                                                                                                                                                               | Käyttäjä haluaa lisätä sähköpostitietoja tietokantaan jossa lisänä on sähköpostiin assosiaationa nimi taikka nimimerkki. |
| Tietojen tarkastelu.   | Sivuston avatessa käyttäjällä on saatavissa painike(get users/fetch users, joku kuvaava termi.) jota painaessaan sovellus paljastaa tietokannan sisällön selkeästi luettavassa muodossa jos tietokannassa on sisältöä (!Tulisiko olla ilmoitus jos tietokanta tyhjä?)                                                                                                                                                                                                                                                                         | Käyttäjä ei halua muokata tietokantaa vaan tarkastella sen sisältöä (!Kannattaako tämä toiminto olla napin takana?)      |
| Tietojen muokkaaminen. | Käyttäjä avaa sivuton jonka jälkeen hän avaa tietokannan tarkastelun kuten yllä mainitaan. Jos tietokannassa on sisältöä, tietokannan arvojen lopuksi käyttäjälle ilmestyy napit "delete" ja "edit". Edit näppäintä painamalla käyttäjä saa mahdollisuuden muokata arvoa johon edit näppäin oli liitetty. Muokkaaminen tapahtuu samalla tavalla kuin tiedon lisääminen(katso tapaus 1). Käyttäjän lisättyä taikka poistettua tietoa antaa ohjelma vastauksen "User updated", jos tietoa muokattiin ja "User deleted", jos tietoa poistettiin. | Käyttäjä voi muokata tietokannan tietoja tarpeidensa mukaan.                                                             |

## 3. Prototyyppi. 

![[Pasted image 20250423124538.png]]

Prototyypin tärkeimmät ominaisuudet:
- Kaksi kenttää, joissa on "Nimi" ja "Sähköposti" joihin käyttäjä voi lisätä tietoja.(Kuvaava termi kenttiin!)
- "Lisää" nappi joka laittaa(POST) annetun tiedon tietokantaan. 
- Näppäin jonka avulla tietokannan sisältöä voidaan tarkastella(Kuvaava nimi napille).
- Näppäin tietojen muokkaamiselle. Näppäin tulee olla sijoitettuna siten että käyttäjä ymmärtää mitä tietoa ollaan muokkaamassa.
- Näppäin tietojen poistamiselle. Näppäin tulee olla sijoitettuna siten että käyttäjä ymmärtää mitä tietoa ollaan poistamassa.
- Käyttäjällä on mahdollisuus ladata tietoa(tässä vaiheessa (Ctrl + C) + (Ctrl + V).)
- Painikkeiden kuvaukset ovat selkeitä ja intuitiivisia. 
- Sivuston tulee olla responsiivinen erilaisia laitteita varten. 
- Sisuston värimaailman tulee olla selkeä näkörajoituksista huolimatta.
- Sivuston tulee olla reaktiivinien, jotta käyttäjä saa välittömän tiedon sivuston toiminnasta.
Päätökset sivuston ulkonäöstä ovat avoimia muutoksille. 

## 4.Tietoarkkitehtuuri ja tekninen suunnittelu.

1. Tietokannan Rakenne: 
	- Tietokanta sisältää seuraavat sarakkeet: ID, name, email.
2. Sivuston rakenne:
	- Etusivu, jossa toiminntot ovat saatavilla.
	- "Lisää" toiminto.
	- "Muokkaa" toiminto olemassa olevaan tietoon liittyen.
	- "Poista" toiminto olemassa olevaan tietoon liittyen.
Sivusto luodaan käyttämällä Rust-ohjelmoinitkieltä ja Rocket frameworkkia. 
Sivuston tietokanta tulee olla PostgreSQL- tietokantaa käyttämällä.
Sivustolla tulee olla CRUD toiminnot joiden avulla tietokannan hallinta on yksinkertaista.
Kehitys tulee tapahtua pilvipalvelussa(Google cloud), mutta ajankäytön puitteissa voidaan tyytyä paikalliseen toimintaan.

## 5. Projektin hallinta.

Dokumentointiin käytän apuna Obsidian [Obsidian - Sharpen your thinking](https://obsidian.md/) muistiinpanotyökalua joka luo MD tiedoston. tämä tiedosto on hieman helpompi täyttää ja lopulta tieto lisätään paikallisen koneen VS-coden kautta GitHubiin. 

Projektin versionhallinnassa käytän GitHubia. 

Teen projektia yksin, joten ainoat dokumentaatioon käytettävät polut ovat opettajalle olevat:
1. Itslearning, tehtävien palautukseen GitHub- linkki
2. Sähköposti, jonka kautta viestin suoraan opettajan kanssa. 
3. Teams. Tämä on vaihtoehtoinen polku ottaa yhetyttä opettajaan. 

Jos tarvitsen tehtävien tekemisessä vertaistukea muilta oppilailta, käytän Discord palvelua.

## 6. Käyttäjätestaus.

Projektille luodaan yksikkötestejä jotka tarkastaa sivuston toimivuutta. Esimerkkejä:
1. tuplaklikkaus ei lisää tietokantaan useampaa arvoa.
2. poistaminen poistaa yhden arvon.
Lisäksi tulee olla käytettävyys testejä, joiden avulla tarkastellaan sivuston toimintoja manuaalisesti. Esimerkiksi:
3. Kuvaus joka on kovin pitkä ei mene ruudun ulkopuolelle.
4. Painikkeilla on responsiivisuus(painaessa painikkeen väri muuttuu)
End 2 End testaus. Luodaan testejä jotka simuloi käyttäjätilanteita. Esim
5. Testi luo arvon, tarkastaa sen olemassaolon, muokkaa sitä, tarkastaa muutoksen. poistaa tiedon ja tarkastaa tiedon poistumisen. 
Kuormitustestaus jonka avulla voidaan simuloida usean käyttäjän samanaikaista toimintaa. (tämä valikoitui)

## 7. Tiedon lataaminen

Projektin yksi tavoitteista olisi kyetä lataamaan tieto MarkDown muotoon valmiiksi taulukoksi, jotta GitHubiin työtuntien kirjaus olisi helpooa ja vaivatonta. Kuitenkin tämä toiminto on toissijainen ja mahdollisesti tiedon lataaminen ja sen latauksen muoto on alttiita muutoksille projektin aikana.