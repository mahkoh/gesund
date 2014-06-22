use tox::core::{Tox};

pub fn bootstrap() -> Tox {
    let tox = Tox::new(true).unwrap();

    let addresses = [
        ("192.254.75.98",   33445, "951C88B7E75C867418ACDB5D273821372BB5BD652740BCDF623A4FA293E75D2F"),
        ("144.76.60.215",   33445, "04119E835DF3E78BACF0F84235B300546AF8B936F035185E2A8E9E0A67C8924F"),
        ("23.226.230.47",   33445, "A09162D68618E742FFBCA1C2C70385E6679604B2D80EA6E84AD0996A1AC8A074"),
        ("37.187.20.216",   33445, "4FD54CFD426A338399767E56FD0F44F5E35FA8C38C8E87C8DC3FEAC0160F8E17"),
        ("54.199.139.199",  33445, "7F9C31FE850E97CEFD4C4591DF93FC757C7C12549DDD55F8EEAECC34FE76C029"),
        ("109.169.46.133",  33445, "7F31BFC93B8E4016A902144D0B110C3EA97CB7D43F1C4D21BCAE998A7C838821"),
        ("192.210.149.121", 33445, "F404ABAA1C99A9D37D61AB54898F56793E1DEF8BD46B1038B9D822E8460FAB67"),
    ];

    for &(ip, port, key) in addresses.iter() {
        let ip = ip.to_string();
        let key = box from_str(key).unwrap();
        let _ = tox.bootstrap_from_address(ip, true, port, key);
    }

    let groupbot_addr =
        "56A1ADE4B65B86BCD51CC73E2CD4E542179F47959FE3E0E21B4B0ACDADE51855D34D34D37CB5";
    let groupbot = box from_str(groupbot_addr).unwrap();
    tox.add_friend(groupbot, "hi".to_string());

    tox
}
