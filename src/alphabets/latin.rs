use crate::utils::is_stop_char;
use crate::Lang;
use crate::core::LowercaseText;

const AFR: &'static str = "abcdefghijklmnopqrstuvwxyzáèéêëíîïóôúû";
const AKA: &'static str = "abdefghiklmnoprstuwyɔɛ";
const AZE: &'static str = "abcdefghijklmnopqrstuvxyzçöüğışə̇";
const CAT: &'static str = "abcdefghijklmnopqrstuvwxyz·àçèéíïòóúü";
const CES: &'static str = "abcdefghijklmnopqrstuvwxyzáéóúýčďěňřšťůž";
const DAN: &'static str = "abcdefghijklmnopqrstuvwxyzåæø";
const DEU: &'static str = "abcdefghijklmnopqrstuvwxyzßäöü";
const ENG: &'static str = "abcdefghijklmnopqrstuvwxyz";
const EPO: &'static str = "abcdefghijklmnoprstuvzĉĝĥĵŝŭ";
const EST: &'static str = "abcdefghijklmnopqrstuvwxyzäõöü";
const FIN: &'static str = "abcdefghijklmnopqrstuvwxyzäöšž";
const FRA: &'static str = "abcdefghijklmnopqrstuvwxyzàâçèéêëîïôùûüÿœ";
const HRV: &'static str = "abcdefghijklmnopqrstuvwxyzćčđšž";
const HUN: &'static str = "abcdefghijklmnopqrstuvwxyzáéíóöúüőű";
const IND: &'static str = "abcdefghijklmnopqrstuvwxyz";
const ITA: &'static str = "abcdefghijklmnopqrstuvwxyzàèéìòù";
const JAV: &'static str = "abcdefghijklmnopqrstuvwxyzèé";
const LAT: &'static str = "abcdefghijklmnopqrstuvwxyz";
const LAV: &'static str = "abcdefghijklmnopqrstuvwxyzāčēģīķļņōŗšūž";
const LIT: &'static str = "abcdefghijklmnopqrstuvwxyząčėęįšūųž";
const NLD: &'static str = "abcdefghijklmnopqrstuvwxyzàèéëïĳ";
const NNO: &'static str = "abcdefghijklmnopqrstuvwxyzåæø";
const NOB: &'static str = "abcdefghijklmnopqrstuvwxyzåæø";
const POL: &'static str = "abcdefghijklmnopqrstuvwxyzóąćęłńśźż";
const POR: &'static str = "abcdefghijklmnopqrstuvwxyzàáâãçéêíóôõú";
const RON: &'static str = "abcdefghijklmnopqrstuvwxyzâîăşţ";
const SLK: &'static str = "abcdefghijklmnopqrstuvwxyzáäéíóôúýčďĺľňŕšťž";
const SLV: &'static str = "abcdefghijklmnopqrstuvwxyzčšž";
const SNA: &'static str = "abcdefghijklmnopqrstuvwxyz";
const SPA: &'static str = "abcdefghijklmnopqrstuvwxyz¡¿áéíñóúü";
const SWE: &'static str = "abcdefghijklmnopqrstuvwxyzäåö";
const TUK: &'static str = "abdefghijklmnoprstuwyzäçöüýňşž";
const TUR: &'static str = "abcdefghijklmnopqrstuvwxyzçöüğış̇";
const UZB: &'static str = "abcdefghijklmnopqrstuvxyzʻ";
const VIE: &'static str = "abcdefghijklmnopqrstuvwxyzàáâãèéêìíòóôõùúýăđĩũơưạảấầẩẫậắằẳẵặẹẻẽếềểễệỉịọỏốồổỗộớờởỡợụủứừửữựỳỵỷỹ";
const YOR: &'static str = "abcdefghijklmnoprstuvwyzàáèéìíòóùúńɔɛ̀́ṣẹọ";
const ZUL: &'static str = "abcdefghijklmnopqrstuvwxyz";

fn get_lang_chars(lang: Lang) -> Vec<char> {
    let alphabet = match lang {
                Lang::Afr => AFR,
                Lang::Aka => AKA,
                Lang::Aze => AZE,
                Lang::Cat => CAT,
                Lang::Ces => CES,
                Lang::Dan => DAN,
                Lang::Deu => DEU,
                Lang::Eng => ENG,
                Lang::Epo => EPO,
                Lang::Est => EST,
                Lang::Fin => FIN,
                Lang::Fra => FRA,
                Lang::Hrv => HRV,
                Lang::Hun => HUN,
                Lang::Ind => IND,
                Lang::Ita => ITA,
                Lang::Jav => JAV,
                Lang::Lat => LAT,
                Lang::Lav => LAV,
                Lang::Lit => LIT,
                Lang::Nld => NLD,
                Lang::Nno => NNO,
                Lang::Nob => NOB,
                Lang::Pol => POL,
                Lang::Por => POR,
                Lang::Ron => RON,
                Lang::Slk => SLK,
                Lang::Slv => SLV,
                Lang::Sna => SNA,
                Lang::Spa => SPA,
                Lang::Swe => SWE,
                Lang::Tuk => TUK,
                Lang::Tur => TUR,
                Lang::Uzb => UZB,
                Lang::Vie => VIE,
                Lang::Yor => YOR,
                Lang::Zul => ZUL,

        _ => panic!(format!("No alphabet for {}", lang)),
    };
    alphabet.chars().collect()
}

pub fn alphabet_calculate_scores(text: &LowercaseText) -> Vec<(Lang, f64)> {
    let mut raw_scores = vec![
                (Lang::Afr, 0i32),
                (Lang::Aka, 0i32),
                (Lang::Aze, 0i32),
                (Lang::Cat, 0i32),
                (Lang::Ces, 0i32),
                (Lang::Dan, 0i32),
                (Lang::Deu, 0i32),
                (Lang::Eng, 0i32),
                (Lang::Epo, 0i32),
                (Lang::Est, 0i32),
                (Lang::Fin, 0i32),
                (Lang::Fra, 0i32),
                (Lang::Hrv, 0i32),
                (Lang::Hun, 0i32),
                (Lang::Ind, 0i32),
                (Lang::Ita, 0i32),
                (Lang::Jav, 0i32),
                (Lang::Lat, 0i32),
                (Lang::Lav, 0i32),
                (Lang::Lit, 0i32),
                (Lang::Nld, 0i32),
                (Lang::Nno, 0i32),
                (Lang::Nob, 0i32),
                (Lang::Pol, 0i32),
                (Lang::Por, 0i32),
                (Lang::Ron, 0i32),
                (Lang::Slk, 0i32),
                (Lang::Slv, 0i32),
                (Lang::Sna, 0i32),
                (Lang::Spa, 0i32),
                (Lang::Swe, 0i32),
                (Lang::Tuk, 0i32),
                (Lang::Tur, 0i32),
                (Lang::Uzb, 0i32),
                (Lang::Vie, 0i32),
                (Lang::Yor, 0i32),
                (Lang::Zul, 0i32),

    ];

    let max_raw_score = text.chars().filter(|&ch| !is_stop_char(ch)).count();

    for (lang, score) in &mut raw_scores {
        let alphabet = get_lang_chars(*lang);

        for ch in text.chars() {
            if is_stop_char(ch) {
                continue;
            };
            if alphabet.contains(&ch) {
                *score += 1;
            } else {
                *score -= 1;
            }
        }
    }

    raw_scores.sort_by(|a, b| b.1.cmp(&a.1));

    let raw_scores: Vec<(Lang, usize)> = raw_scores
        .into_iter()
        .map(|(l, s)| {
            let score = if s < 0 { 0usize } else { s as usize };
            (l, score)
        })
        .collect();

    let mut normalized_scores = vec![];

    for &(lang, raw_score) in &raw_scores {
        let normalized_score = raw_score as f64 / max_raw_score as f64;
        normalized_scores.push((lang, normalized_score));
    }

    normalized_scores
}
