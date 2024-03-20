use blue_archive::{types::summons::Skill, Language};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    for summon in blue_archive::fetch_all_summons(Language::English).await? {
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        println!("Passive Skills for {}", summon.id);

        for skill in summon.skills {
            if let Skill::Passive(passive_skill) = skill {
                println!("{:#?}", passive_skill.icon())
            }
        }
    }

    Ok(())
}
