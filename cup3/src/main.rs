use std::io;
use rand::Rng;

fn main()
{
    let mut point = 0;
    let mut count = 0;
    loop{
        point+= game();
        count+=1;
        let continue_game = get_continue_game();
        if !continue_game
        {
            break;
        }
    } 
    println!("You're point is {}", point);
    println!("You're count is {}", count);
    println!("You're score is {} %", (point as f64 / count as f64) * 100.0);
}
fn get_continue_game() -> bool
{
    //入力文字列がcから始まればtrueを返す
    println!("Are you contine game ? contine/ no");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        if input.starts_with("c") || input.starts_with("C")
        {
            return true;
        }
        else if input.starts_with("n") || input.starts_with("N")
        {
            return false;
        }
        else
        {
            continue;
        }
    }
}
fn game() -> i32 {
    //1~3の数字を当てるゲーム
    //正解を1以上3以下の数字ランダムに決定する。


    let answer = rand::thread_rng().gen_range(1..=3);

    //プレイヤーに標準出力から数字を入力してもらう。
    let first_input_number = get_player_num();

    //ヒントとしてプレーヤーが入力した数字以外で正解ではない数字を表示する。
    let hint = get_hint(answer, first_input_number);
    //output hint
    println!("hint: {} is not answer", hint);

    //プレイヤーは数字を変更するか、そのままにするか選ぶ
    println!("Do you want to change your number? y/n");
    let change = get_change();
    //プレイヤー変更した数字を計算する。
    let second_input_number = get_second_input(change, first_input_number, hint);
    //output second_input_number
    println!("You're final input number is : {}", second_input_number);
    //正解と比較する
    if second_input_number == answer
    {
        println!("You win!");
        return 1;
    }
    else
    {
        println!("The answer is {}", answer);
        println!("You lose...");
        return 0;
    }

}

fn get_second_input(change : bool, first_input_number : i32, hint : i32) -> i32
{
    if change
    {
        for i in 1..=3 
        {
            if i != hint && i != first_input_number
            {
                return i;
            }
        }
    }

    return first_input_number;
    
}
fn get_change() -> bool
{
    //プレイヤーがyかnを入力する。
    //yで始まる文字ならtrue
    //nで始まる文字ならfalse
    //を返す。大文字小文字は区別しない
    loop
    {
        println!("Please input y or n.");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        if input.starts_with("y") || input.starts_with("Y")
        {
            return true;
        }
        else if input.starts_with("n") || input.starts_with("N")
        {
            return false;
        }
        else
        {
            continue;
        }
    }
}

fn get_hint(answer : i32 , input : i32) -> i32
{
    //１〜３でinputとanswer以外の数字を返す。
    
    for i in 1..=3 
    {
        if i != answer && i != input
        {
            return i;
        }
    }
    return 0;
}

fn get_player_num() -> i32
{
    //プレイヤーが1~3の数字を入力する。
    //1~3以外を入力したばあい再度入力する。
    loop
    {
        println!("Please input your input.");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if input < 1 || input > 3
        {
            continue;
        }
        else
        {
            return input;
        }
    }
}

