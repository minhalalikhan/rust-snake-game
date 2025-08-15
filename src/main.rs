use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{Event, KeyCode, KeyEvent, poll, read},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode, size,
    },
};

use rand::Rng;

use std::{
    collections::VecDeque,
    io::{Result, Write, stdin, stdout},
    time::{Duration, Instant},
};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    left,
    right,
    up,
    down,
}

fn main() -> Result<()> {
    println!("Hello, Crossterm!");

    let mut stdout = stdout();

    execute!(stdout, EnterAlternateScreen, Hide)?;
    enable_raw_mode()?;

    // Clear screen
    execute!(stdout, Clear(ClearType::All))?;

    main_menu();
    start_game()?;

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, Show)?;
    Ok(())
}

fn main_menu() {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All)).unwrap();

    execute!(stdout, MoveTo(10, 5), Print("Welcome to Rust Snake CLI!")).unwrap();
    execute!(stdout, MoveTo(10, 6), Print("Press any key to start...")).unwrap();
    stdout.flush().unwrap();

    // Wait for any key press
    let _ = read().unwrap();
}

fn start_game() -> Result<()> {
    let mut rng = rand::thread_rng();
    let mut stdout = stdout();

    let (cols, rows) = size()?;

    // range for the playable area
    let min_x = 2;
    let min_y = 1;
    let max_x = cols - 3;
    let max_y = rows - 2;

    let playarea = (cols - 4) * (rows - 2);
    let mut dir: Direction = Direction::right;

    // initialize snake from the center of the screen
    let mut snake: VecDeque<Point> = VecDeque::from([Point {
        x: cols / 2,
        y: rows / 2,
    }]);

    // screen will be updated once  rate time has crosssed last tick
    let mut last_tick = Instant::now();

    //  at what rate the screen will be updated
    let tick_rate = Duration::from_millis(100);

    // score
    let mut score = 0;

    let mut food = Point {
        x: rng.gen_range(min_x..=max_x),
        y: rng.gen_range(min_y..=max_y),
    };

    draw_border()?;

    loop {
        let p = Point {
            x: rand::thread_rng().gen_range(min_x..=max_x),
            y: rand::thread_rng().gen_range(min_y..=max_y),
        };

        // is food is not inside snake place it else repeat this loop
        if !snake.iter().any(|&s| s == p) {
            food = p;

            break;
        }
    }

    loop {
        //  catch new direction
        if poll(Duration::from_millis(0))? {
            match read()? {
                Event::Key(KeyEvent { code, .. }) => {
                    if code == KeyCode::Char('q') {
                        game_over()?;
                        break;
                    }

                    if code == KeyCode::Char('p') {
                        pause_game()?;
                    }

                    dir = match (dir, code) {
                        (Direction::down, KeyCode::Up)
                        | (Direction::up, KeyCode::Down)
                        | (Direction::left, KeyCode::Right)
                        | (Direction::right, KeyCode::Left) => dir,
                        (_, KeyCode::Up) => Direction::up,
                        (_, KeyCode::Down) => Direction::down,
                        (_, KeyCode::Left) => Direction::left,
                        (_, KeyCode::Right) => Direction::right,
                        _ => dir,
                    }
                }

                _ => {}
            }
        }

        // update the screen if the tick rate has crossed
        if last_tick.elapsed() > tick_rate {
            last_tick = Instant::now();

            // next head position

            let mut head = *snake.front().unwrap();

            head = match dir {
                Direction::up => Point {
                    x: head.x,
                    y: head.y - 1,
                },
                Direction::down => Point {
                    x: head.x,
                    y: head.y + 1,
                },
                Direction::left => Point {
                    x: head.x - 1,
                    y: head.y,
                },
                Direction::right => Point {
                    x: head.x + 1,
                    y: head.y,
                },
            };

            // end game if snake hits the border
            if head.x < min_x || head.x > max_x || head.y < min_y || head.y > max_y {
                game_over()?;
                break;
            };

            // // end game if snake hits itself
            if snake.iter().any(|&p| p == head) {
                game_over()?;
                break;
            }

            // add new head to snake
            snake.push_front(head);

            // snake eats the food

            if head == food {
                score += 1;

                //  place new food on board

                loop {
                    let p = Point {
                        x: rand::thread_rng().gen_range(min_x..=max_x),
                        y: rand::thread_rng().gen_range(min_y..=max_y),
                    };

                    // is food is not inside snake place it else repeat this loop
                    if !snake.iter().any(|&s| s == p) {
                        execute!(stdout, MoveTo(food.x, food.y), Print(" "))?;

                        food = p;

                        break;
                    }
                }
            } else {
                // if snake doesnt eat , we remove last element
                let removed_tail = snake.pop_back();

                if let Some(tail) = removed_tail {
                    execute!(stdout, MoveTo(tail.x, tail.y), Print(" "))?;
                }
            }

            draw_frame(&snake, &food)?;

            if score == playarea {
                game_won()?
            }
        }
    }

    Ok(())
}

fn pause_game() -> Result<()> {
    // let mut stdout = stdout();

    // Wait for any key press
    let _ = read()?;

    Ok(())
}

fn draw_frame(snake: &VecDeque<Point>, food: &Point) -> Result<()> {
    let mut stdout = stdout();

    //  print food
    execute!(stdout, SetBackgroundColor(Color::DarkGreen))?;
    execute!(stdout, MoveTo(food.x, food.y), Print(" "))?;

    // // print head
    execute!(stdout, SetBackgroundColor(Color::DarkYellow))?;

    if let Some(head) = snake.front() {
        execute!(stdout, MoveTo(head.x, head.y), Print(" "))?;
    }

    execute!(stdout, SetBackgroundColor(Color::Yellow))?;
    for seg in snake.iter().skip(1) {
        execute!(stdout, MoveTo(seg.x, seg.y), Print(" "))?;
    }
    execute!(stdout, ResetColor)?;
    Ok(())
}

fn draw_border() -> Result<()> {
    let (cols, rows) = size()?;
    let mut stdout = stdout();

    let border_chhar = " ";

    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, SetBackgroundColor(Color::DarkGrey))?;

    // Top border
    execute!(
        stdout,
        MoveTo(0, 0),
        Print(border_chhar.repeat(cols as usize))
    )?;
    // Bottom border
    execute!(
        stdout,
        MoveTo(0, rows - 1),
        Print(border_chhar.repeat(cols as usize))
    )?;

    // Left and right borders
    for row in 1..rows - 1 {
        execute!(stdout, MoveTo(0, row), Print(border_chhar.repeat(2)))?;
        execute!(stdout, MoveTo(cols - 2, row), Print(border_chhar.repeat(2)))?;
    }
    execute!(stdout, ResetColor)?;
    // execute!(stdout, MoveTo(2, 1))?;
    Ok(())
}

fn game_over() -> Result<()> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All)).unwrap();

    execute!(stdout, MoveTo(10, 5), Print("GAME OVER !!")).unwrap();
    execute!(stdout, MoveTo(10, 6), Print("Better luck next time...")).unwrap();
    loop {
        if poll(Duration::from_millis(10))? {
            if let Event::Key(_) = read()? {
                break;
            }
        }
    }

    Ok(())
}

fn game_won() -> Result<()> {
    loop {
        if poll(Duration::from_millis(10))? {
            if let Event::Key(_) = read()? {
                break;
            }
        }
    }

    Ok(())
}
