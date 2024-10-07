use std::collections::HashMap;
#[macro_use]
use crate::parseerr;

#[derive(Debug, PartialEq)]
enum VarState {
    None,       // 変数の状態なし
    StartVar,   // 変数名の開始
    StartValue, // 値の定義開始
}

pub fn parse(parse_text: &str) {
    let mut comment = false; // コメント中かどうかを示すフラグ
    let mut vars = HashMap::new(); // 定義された変数を格納するハッシュマップ
    let mut var_state = VarState::None; // 変数の定義状態を追跡
    let mut temp_var = String::new(); // 変数名を一時的に保持
    let mut temp_value = String::new(); // 変数の値を一時的に保持
    let mut temp_text = String::new(); // その他の文字を一時的に保存
    let mut field: Vec<String> = Vec::new(); // フィールドを保持するベクター
    let mut block = 0; // ブロックの深さを管理
    let mut no_field = String::new(); // フィールドが定義されていない状態を保持
    let mut field_bool = false; // フィールド状態になっているかどうかのフラグ
    let mut db_quo = false; // ダブルクォーテーション内かどうかのフラグ
    let mut main = false; // main関数の存在フラグ
    let mut text = String::new(); // 文字列を一時的に保存する
    let mut main_field = false;
    let mut text_end = false;
    let mut line = 1;
    let mut in_puts = false; // puts命令の処理中かどうかを示すフラグ
    let mut puts_content = String::new(); // puts命令の内容を保持

    // 入力テキストの各文字を処理
    for c in parse_text.chars() {
        if comment {
            // コメント中の場合、改行が来るまで無視
            if c == '\n' {
                comment = false; // 改行でコメント終了
            }
            continue;
        }
        if c == '\n'{
            line += 1;
        }

        match c {
            '{' => { // ブロックの開始
                if !db_quo {
                    block += 1;
                    if main {
                        main_field = true;
                    }
                }
            }
            '}' => {  // ブロックの終了
                if !db_quo {
                    if block == 0{
                        parseerr!(line, "Unexpected closing brace '}'", "Closing brace without matching opening brace" ,"Unexpected closing brace");
                    }
                    block -= 1;
                    if main_field {
                        main_field = false;
                        main = false;
                    }
                }
            }
            ' ' if !db_quo => continue,       // 空白を無視
            '#' if !db_quo => comment = true, // コメント開始
            '$' if block >= 1 && !db_quo => {
                if var_state != VarState::None{
                    parseerr!(line,"Unexpected variable declaration" ,"Variable declaration must start from a new line", "Unexpected variable declaration");
                }
                // 変数名の開始を検出
                var_state = VarState::StartVar;
                temp_var.clear(); // 新しい変数名を格納する準備
            }
            '"' => {
                if db_quo {
                    db_quo = false;
                    if in_puts {
                        // puts命令の内容を出力
                        println!("{}", puts_content);
                        in_puts = false;
                        puts_content.clear();
                    }
                } else {
                    db_quo = true;
                }
            }
            '=' if block >= 1 && !db_quo => {
                // 変数名が終了し、値の定義を開始
                if var_state != VarState::StartVar{
                    parseerr!(line, "Unexpected '=' character", "Assignment operator must follow a variable name","Unexpected character");
                }
                if var_state == VarState::StartVar {
                    var_state = VarState::StartValue;
                    temp_value.clear(); // 新しい値を格納する準備
                }
            }
            '\n' => {
                if block >= 1 && !db_quo {
                    // 改行で値の定義が終了
                    finalize_variable(&mut vars, &temp_var, &temp_value, &mut var_state);

                    if !temp_text.trim().is_empty() {
                    }
                }
            }
            ':' if block >= 1 && !db_quo && main => {
                if temp_text.trim() != "puts" {
                    parseerr!(line, "Unexpected ':' character", "Colon must follow 'puts' keyword", "Unexpected character");
                }
                in_puts = true;
                temp_text.clear();
            }
            _ => {
                if block >= 1 {
                    if in_puts && db_quo{

                    }
                    // ブロック内での文字の処理
                    match var_state {
                        VarState::StartVar => temp_var.push(c),     // 変数名を追加
                        VarState::StartValue => temp_value.push(c), // 変数の値を追加
                        _ => {
                            if in_puts && db_quo {
                                puts_content.push(c); // puts命令の内容を保持
                            } else {
                                temp_text.push(c); // その他の文字を保持
                            }
                        }
                    }
                } else if !db_quo {
                    no_field.push(c); // フィールドが定義されていない状態の文字を保持
                }
            }
        }

        if db_quo {
            if c == '"' {
                continue;
            }
            text.push(c); // ダブルクォーテーション内の文字を追加
        }
        // フィールドの確認
        if no_field.trim() == "field" {
            field_bool = true;
            no_field.clear();
        }

        if field_bool && text.trim() == "main" {
            main = true;
            field_bool = false;
        }

        // ここでフィールドの確認
        if let Some(field_name) = field.get(0) {
            // "main"フィールドの確認
            if field_name == "main" {
                main = true; // mainが存在する
            }
        }
    }

    // 最後の変数も登録する
    if var_state == VarState::StartValue {
        vars.insert(temp_var.trim().to_string(), temp_value.trim().to_string());
    }
}

// 変数を最終登録する関数
fn finalize_variable(
    vars: &mut HashMap<String, String>,
    temp_var: &str,
    temp_value: &str,
    var_state: &mut VarState,
) {
    if *var_state == VarState::StartValue {
        // 変数とその値をハッシュマップに追加
        vars.insert(temp_var.trim().to_string(), temp_value.trim().to_string());
        *var_state = VarState::None; // 状態をリセット
    }
}

