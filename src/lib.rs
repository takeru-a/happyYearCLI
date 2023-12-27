pub mod api;

use rand::Rng;
use trust_dns_resolver::config::*;
use trust_dns_resolver::TokioAsyncResolver;
use anyhow::{Result, Ok};
use anyhow::anyhow;
use std::net::{IpAddr, Ipv4Addr};

pub fn get_message(message_type: usize) -> String {
    // todo ここに文章を追加していく , エラー処理も追加する
    // データ
    let message_list =  vec![  
        r#"
        Happy New Year!
        "#,
    ];

    return message_list[message_type].to_string();
}

// おみくじの結果を返す
pub fn get_omikuji() -> String {
    // todo ここにおみくじの内容記載 , エラー処理も追加する
    let mut rng = rand::thread_rng();
    let omikuji = rng.gen_range(0..=5);
    let omikuji_list = vec![
        "大吉",
        "吉",
        "中吉",
        "小吉",
        "末吉",
        "凶",
    ];
    return omikuji_list[omikuji].to_string();
}

// グローバルIPアドレスを取得する
pub async fn get_global_ip() -> Result<String> {

    // OpenDNSのresolver1.opendns.com(IPアドレス)を設定
    let resolver = TokioAsyncResolver::tokio(
        ResolverConfig::from_parts(
            None,
            vec![],
            NameServerConfigGroup::from_ips_clear(&[IpAddr::V4(Ipv4Addr::new(208, 67, 222, 222))], 53,true),
        ),
        ResolverOpts::default());
    
    // 'myip.opendns.com' のDNSクエリを実行する
    let response = resolver.lookup_ip("myip.opendns.com.").await?;
    let ip = response.iter().next().ok_or_else(|| anyhow!("IPアドレスを取得できませんでした"))?;
    Ok(ip.to_string())
}

pub fn get_art(art: usize) -> String {

    // todo ここにアスキーアートを追加していく , エラー処理も追加する
    // 龍、鏡餅、富士山
    // データ
    let art_list =  vec![  
        r#"
        J8XjZJYO....gY"<          .                 .`  ..
        -MdW2yZW7dj7d9&Jga,          j.   ,,       . Mt .F`
       .M8M?JF..74#agxdN,  !      `   dp  -\       ].Hh+g. .     `       ....      .;
        TMdHmfdiuMjMJM#NMa...  `      (N-.M% .     ?""OMN,.#        ` ....T,,x .   dD
         ,M#QW8?JH,W,M]W,   .     s  (MNMM}  ('       JM,?^ ...?uJ.7Q, ?o(L(N(a.4, T5
           (MMfQmaH,v/75i. . . ...TEdMMM8Mpggr    .+gUH?5M5(.,dMaJMa.M/hn5,N,YgTNJN,HL  .,
             ,MNT6mW!.     10j&g&MQS,"_rJX.      .O&JGsYGVWRTgqKBdJBqa,,4JMNdMgHNJmWJB dP
      `       jQxEQJM!    ., jVVBZ51w(CdJBd       (XzZ1Jj#QMqS&SM$NZgHMJm,YNJWNMhHmHJd(N,  (  `  `
         `    ?M@#+JMf ....dMHMBMN&KEwHJ8%           ..MG#dNT5eddM?BdSV5aJHa,HNdNdgMN-8N,  .;
             jMNGBgJq.((gB&xgbgZQJBudMdud.     `     .#sd#gJMhSmMJMdMknJ9UgdMMMMMh#M@NWMm  J        `
     `      .dH#Xmp5F..d5MdEVQGJhTBgK?Edm..        .+JdVNWNJNM8gghWNJYTz&jgMM#MMxddkMR(HMc 4..
        `  -MM#JWJnJ=`.#MQJhNMdNN#gKdBggUde.       _.4MPMdMdkggHM#! `  ..gMMmITNZXggMMJNmM,  u
           .g#gdNuJ9=(.@pNgdqMMMdNgdNgGMBuJM.(     ?.ddMd@N?HMMN.       ?O7qMMKHMXMHdNMdMNJF d    `
          .MM8pMD4b, .?#nWgkNgHMJMMMmWMVMMJKMF ;   ,"PNdMd#dfMM@     (.    7"3OgEWUp8WkJmYYY1[
      `   .ggJdMqj.- ?W5pMMJMMM] ?WB&NWMNMEHDNy     -YJNJ8xHMdMMm.     .W.    ?9msW#ejKNg,   ?2
          -MMdhMbyB,,9vMJMbaggD    JMMNMHDMMdfm/     .BdgJNgWMdMMh      .b      .,#K?#WJMMB.  J&
           .gsMmMgJddeM8M#dMMM#     MMMKMSJ9MjW2`    ..BsN(gZgMJMMMm     ?,   ..(MH@! .zBQ#M#Tb.h
        ,MW&wNJhM5gJM#QfHM@`      (gK(gddM4J@~      !@4MvM$XMmHQgN,     _!`  .g8d"..idHd@NMM$.]
      `      MMMMqgg6MNsMMMM         dM$jsH8M@gMt,       7?m8MMJqNdMMM[        ?7WNxTN,...d@-, T9`
              .T"MMMPMM#?"^          ,6Wg7M8dMdJN..       ,.MZ+qg(MNfdMMx       77TML  ?""=
                                 ..  ,MJEH8MqM#gJNJ         _YhWdM+MMNMBaa.       "
        `                  .-<JWWQ9dMdNdMd5MMjMbgda%.         %mHJMPMMNdMM#    .-Q,..
           `            .(JXXuuQSdHugqgd8dEdMMnMFNmH[|        ..dMdM8MMbMMN,  .MM! MMdN,
                         .?"""4g#egpgJ" .MMNdgNJM5HHgh....   .&(SMsMMqk8MMM#   HM,.M# ?YQ,
        `                      (Jgnm=    UMMM?NNpM@gGMNdb.eQ.Nd5dRjM@MBdedW@..(MMMMMt.M(T!
             `  `               qmqb      ("gMNVdMUMdNdd#dhKjhbNUJMBdMqJM4MgHLZmMMMMB^ 7M;
                               ..WNF..,..   ?MMMNgT@NNddMqN(ggpMmTbM#dmjMEvmh?!(Y"      dt
      `   `         `  `    .,MMMMMNgMD?TN.   (""4MNgBMM(NGgK&mdNdMgNe?qWWM#^  7b..
                            MD_!qMMMMNTMp         ?MMM@NaggHfHggMJMMMFXXdY`
             `   `          ^    ?Mm.  ,M~             TMMMMFMMMM#?"! 4UJ`
         `                     (j.d"   .D                ?"^ ?""!
                       `         .`    ^
            `       `    `
            "#,
    ];

    return art_list[art].to_string();
}
