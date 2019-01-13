use select::document::Document;
use select::predicate::Class;
use std::collections::BTreeMap;
use std::fmt;

static BASE_URL: &'static str = "https://steamcommunity.com/stats/432980/leaderboards/";

lazy_static! {
    static ref STAGES: BTreeMap<&'static str, &'static str> = {
        let mut stages = BTreeMap::new();
        stages.insert("Bricks1", "1412910");
        stages.insert("Castle1", "1124141");
        stages.insert("Control1", "1902184");
        stages.insert("Far1", "1122188");
        stages.insert("Free1", "1122189");
        stages.insert("Globe1", "1144986");
        stages.insert("Holes1", "1124146");
        stages.insert("Hollow1", "1900302");
        stages.insert("Roads1", "1144985");
        stages.insert("Threat1", "1900300");
        stages.insert("Tiles1", "1900297");

        stages.insert("Bricks2", "1412911");
        stages.insert("Castle2", "1156828");
        stages.insert("Control2", "1900299");
        stages.insert("Far2", "1124143");
        stages.insert("Free2", "1124145");
        stages.insert("Globe2", "1124140");
        stages.insert("Holes2", "1124144");
        stages.insert("Hollow2", "1900303");
        stages.insert("Roads2", "1120514");
        stages.insert("Threat2", "1900301");
        stages.insert("Tiles2", "1900298");
        stages
    };
}

pub struct Ranking(pub Vec<Rank>);

impl fmt::Debug for Ranking {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rank_iter = self.0.iter();
        writeln!(
            f,
            "{:^15}{:^5}{:^20}{:^15}",
            "Stage", "Rank", "Player", "Score"
        )?;
        print!("{:?}", rank_iter.next().unwrap());
        for rank in rank_iter {
            print!("\n{:?}", rank);
        }
        write!(f, "")
    }
}

pub struct Rank {
    pub stage: String,
    pub name: String,
    pub rank: usize,
    pub score: String,
}

impl fmt::Debug for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:^15}{:^5}{:^20}{:^15}",
            self.stage, self.rank, self.name, self.score
        )
    }
}

pub fn fetch_ranking(stage: &str) -> Result<Vec<Rank>, reqwest::Error> {
    let stage_id = STAGES.get(stage).unwrap();
    let url = BASE_URL.to_string() + stage_id;
    let body: &str = &reqwest::get(&url)?.text()?;
    let document = Document::from(body);
    let players: Vec<Rank> = document
        .find(Class("lbentry"))
        .enumerate()
        .map(|(rank, player)| {
            let name = player
                .find(Class("player"))
                .next()
                .unwrap()
                .text()
                .trim()
                .to_string();
            let score = player
                .find(Class("score"))
                .next()
                .unwrap()
                .text()
                .trim()
                .to_string();
            Rank {
                stage: stage.to_string(),
                name,
                rank: rank + 1,
                score,
            }
        })
        .collect();
    Ok(players)
}

pub fn fetch_1st_players() -> Result<Ranking, reqwest::Error> {
    Ok(Ranking(
        STAGES
            .keys()
            .map(|stage| fetch_ranking(stage).unwrap().into_iter().next().unwrap())
            .collect(),
    ))
}

pub fn fetch_players_one(first: bool) -> Result<Ranking, reqwest::Error> {
    let ranking = Ranking(
        STAGES
            .keys()
            .filter(|&stage| stage.find("1").is_some())
            .flat_map(|stage| fetch_ranking(stage).unwrap())
            .collect(),
    );
    let ranking = if first {
        Ranking(
            ranking
                .0
                .into_iter()
                .filter(|ref rank| rank.rank == 1)
                .collect(),
        )
    } else {
        ranking
    };
    Ok(ranking)
}

pub fn fetch_players_two(first: bool) -> Result<Ranking, reqwest::Error> {
    let ranking = Ranking(
        STAGES
            .keys()
            .filter(|&stage| stage.find("2").is_some())
            .flat_map(|stage| fetch_ranking(stage).unwrap())
            .collect(),
    );

    let ranking = if first {
        Ranking(
            ranking
                .0
                .into_iter()
                .filter(|ref rank| rank.rank == 1)
                .collect(),
        )
    } else {
        ranking
    };
    Ok(ranking)
}
