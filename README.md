# happyYearCLI
Rustをつかって何か作りたかったので簡単な自作コマンドを作成しました！  
Windows OS 64bitのみ対象です。  
[happyyear.exe](https://github.com/takeru-a/happyYearCLI/blob/main/happyyear.exe)をダウンロードしてください！

```bash
# 例
$ happyyear -t 1 -a 0
```
-tでメッセージを指定できます。
-aで出力するアスキーアートを指定できます。

**コマンドのオプション対応表**

| option | value | description |
|-------------|-------------|-------------|
| -t | 0 | メッセージ1 |
| -t | 1 | メッセージ2 |
| -t | 2 | メッセージ3 |
| -a | 0 | 龍 |
| -a | 1 | 鏡餅 |
| -a | 2 | 富士山 |

## 対応OS
Window 64bit

## 注意事項
このコマンド実行時にセキュリティブッロクが入ると思いますが、使用する場合は許可してください。
このコマンドは内部で使用者のグローバルIPを取得し、それを基にロケーション特定、天気、神社情報取得を行っております。
情報の収集は行っておりませんが、上記のことをご理解した上でご使用ください。

## 機能一覧
- 出力するメッセージを変更できる　✅
- 出力するアスキーアートを変更できる　✅
- 現在時間を表示できる(年月日)　✅
- おみくじ機能 ✅
- ゆっくり出力する ✅
- ロケーション特定、気温、天気 ✅
- 近くの神社を出力する ✅

### ロケーション特定、気温、天気について
グローバルIPアドレス取得→API(ロケーション特定)→天気APIの順にデータを取得している。

## 使用したAPI
- グローバルIPからロケーション特定するAPI
非営利使用は無料でAPIKeyも不要なので以下のAPIを使用させていただいております。
→ https://ip-api.com/
- ロケーション情報から天気を取得するAPI
こちらも非営利使用は無料でAPIKeyも不要なので以下のAPIを使用させていただいております。
→ https://open-meteo.com/

```
WMO 気象解釈コード (WW)
コード	説明
0	晴天
1、2、3	晴れ時々曇り、曇り
45、48	霧と降る霧氷
51、53、55	霧雨: 軽い、中程度、そして濃い強度
56、57	氷結霧雨: 軽くて濃い強度
61、63、65	雨：小雨、中程度、激しい雨
66、67	凍てつく雨：軽くて激しい雨
71、73、75	降雪量: わずか、中程度、激しい
77	雪の粒
80、81、82	にわか雨：小雨、中程度、激しい雨
85、86	雪が少し降ったり、激しく降ったりします
```

- 地図情報取得API(OpenStreetMap)
OpenStreetMap上の情報を取得できます。無料で利用できます。
→ https://wiki.openstreetmap.org/wiki/JA:Overpass_API

## 代替え方法について
今回使用したAPI以外でも同じようなことを実現できるAPIがあったので、紹介いたします。
### グローバルIPアドレス取得
以下のAPIを使用することでグローバルIPを簡単に取得できます。
- https://ipinfo.io/
- https://ipecho.net/
### ロケーション特定
ログインとAPIKey取得が必要ですが、無料でかなりのリクエスト数APIを呼び出すことができます。
- https://ipinfo.io/ (50k/1month)
- https://www.ip2location.io/ (30k/1month)やりたいこと整理
### 天気API
- [アメダス](https://jjwd.info/index.html)
- https://openweathermap.org/
- https://www.weatherapi.com/
### 地図情報取得API
- [google map API](https://developers.google.com/maps/?hl=ja)
- [yahoo map](https://developer.yahoo.co.jp/webapi/map/)  
グローバルIPアドレス取得、ロケーション特定、天気API、地図APIともに他にも色々APIがあるため1つがダメになっても  
変更はできると思います。


### アスキーアートに使用させていただいたサイト
アスキーアート作成ツール  
https://tool-taro.com/image_to_ascii/  
https://rakko.tools/tools/68/  

画像  
https://www.ac-illust.com/main/detail.php?id=1133137&word=%E4%B8%AD%E8%8F%AF%E9%A2%A8%E9%BE%8D%E6%9F%84  
https://www.ac-illust.com/main/detail.php?id=24629683&word=%E9%8F%A1%E9%A4%85  
https://www.ac-illust.com/main/detail.php?id=1081788&word=%E8%8A%9D%E6%A1%9C%E3%81%A8%E5%AF%8C%E5%A3%AB%E5%B1%B1#goog_rewarded  

### 参考
https://rust-cli.github.io/book/index.html  
