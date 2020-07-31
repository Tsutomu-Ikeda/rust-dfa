use std::fmt;
use std::io::{self, Read};

fn get_input_value() -> String {
  let mut buffer = String::new();
  let stdin = io::stdin();
  let mut handle = stdin.lock();

  handle.read_to_string(&mut buffer).unwrap();
  return String::from(buffer.trim());
}

enum States {
  S0,
  S1,
  S2,
  S3,
  S4,
  S5,
  S6,
}

impl fmt::Display for States {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let printable = match *self {
      States::S0 => "S0",
      States::S1 => "S1",
      States::S2 => "S2",
      States::S3 => "S3",
      States::S4 => "S4",
      States::S5 => "S5",
      States::S6 => "S6",
    };
    write!(f, "{}", printable)
  }
}

impl States {
  // 初期状態
  fn new() -> States {
    States::S0
  }

  // 受理状態
  fn is_accept_state(self) -> bool {
    match self {
      States::S0 => true,
      _ => false,
    }
  }

  // 状態遷移関数
  fn next(self, i: char) -> States {
    match self {
      States::S0 => match i {
        'k' => States::S1,
        'r' => States::S3,
        'y' => States::S5,
        'a' => {
          print!("あ");
          States::S0
        }
        'o' => {
          print!("お");
          States::S0
        }
        _ => States::S6,
      },
      States::S1 => match i {
        'k' => {
          print!("っ");
          States::S1
        }
        'y' => States::S2,
        'a' => {
          print!("か");
          States::S0
        }
        'o' => {
          print!("こ");
          States::S0
        }
        _ => States::S6,
      },
      States::S2 => match i {
        'a' => {
          print!("きゃ");
          States::S0
        }
        'o' => {
          print!("きょ");
          States::S0
        }
        _ => States::S6,
      },
      States::S3 => match i {
        'r' => {
          print!("っ");
          States::S3
        }
        'y' => States::S4,
        'a' => {
          print!("ら");
          States::S0
        }
        'o' => {
          print!("ろ");
          States::S0
        }
        _ => States::S6,
      },
      States::S4 => match i {
        'a' => {
          print!("りゃ");
          States::S0
        }
        'o' => {
          print!("りょ");
          States::S0
        }
        _ => States::S6,
      },
      States::S5 => match i {
        'y' => {
          print!("っ");
          States::S5
        }
        'a' => {
          print!("や");
          States::S0
        }
        'o' => {
          print!("よ");
          States::S0
        }
        _ => States::S6,
      },
      States::S6 => match i {
        _ => States::S6,
      },
    }
  }
}

fn translate(input: String) {
  print!("{}: ", input);
  let mut state = States::new();

  for i in input.chars() {
    state = state.next(i)
  }

  print!("\n");

  println!("最終遷移状態: {}", state);

  if state.is_accept_state() {
    println!("正常に変換されました。")
  } else {
    println!("正常に変換されませんでした。不正な入力です。")
  }
}

fn main() -> io::Result<()> {
  let input = get_input_value();

  for line in input.split("\n") {
    translate(String::from(line));
  }
  Ok(())
}
