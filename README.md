# AtCoder Upcoming Contests
1日1回AtCoderで予定されたコンテストをスクレイピングするWebサーバー実装です。  
Shuttleでデプロイしています。(https://atcoder-upcoming-contests-cs7x.shuttle.app/)

## レスポンス
`GET /`
```ts
{
  start_time: string, // 開始時刻(UTCのISO8601形式)
  name: string, // コンテスト名
  duration: number, // 時間(分)
  rated_range: string,  // AtCoderのレーティング範囲
  url: string, // コンテストURL
}[]
```
