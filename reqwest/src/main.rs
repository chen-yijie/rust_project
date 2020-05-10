
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client_get = reqwest::blocking::Client::new();
    let res_get = client_get.get("https://www.baidu.com")
        .body("the exact body that is sent")
        .send();

    println!( "{:#?}", res_get );

    let client = reqwest::blocking::Client::new();
    let res = client.post("https://android.clients.google.com/auth")    
        .header( "Host", "android.clients.google.com" )
        .header( "Accept", "*/*" )
        .header( "Accept-Encoding", "deflate, gzip" )
        .header( "User-Agent", "Mozilla/5.0 (sailfish NOF26V) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36" )
        .header( "ContentType", "application/x-www-form-urlencoded" )
        .header( "device", "3ecba8ef4aa9041c" )
        .header( "Content-Type", "application/x-www-form-urlencoded" )
        .body( "androidId=3879a8b18374c6d7&lang=en_US&sdk_version=19&callerSig=38918a453d07199354f8b19af05ec6562ced5788&Email=ljjzz314q@gmail.com&EncryptedPasswd=AFcb4KQqXVIXeZ4D9WPlrkVlqqjVc9eedrgd4jdvhEB3rNWKh0Nnw-oFXgkyEvibpgb-iM5rTtjtq2Zc1pZ5Sz-w8wZpnBtzJEzy-9qjf24z5CiJFPT-YjoIrtOOxPsRvibIvYrHK9TzFdKFatEo0P-jwZFsSn-N0VCk0PAltRqIycwTFA==&add_account=1&callerPkg=com.google.android.gms&get_accountid=1&service=ac2dm" )
        .send()?;
    println!( "{:#?}", res );


    Ok(())
}