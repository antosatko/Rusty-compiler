import "std.time" as time
import "std.window" as win

pub const S_WIDTH: uint = 650
const S_HEIGHT: uint = 400

type danda<T(auto), A> = mem.HashMap<Danda.nevim<ahoj>, Danda2>



struct Ball<Generic(tr)> {
    x: float,
    y: float,
    r: float,
    xs: float,
    ys: float,
}


enum Sides {
    Left = 50,
    Right = 600,
}

struct Player {
    side: Sides,
    y: float,
    w: float,
    h: float,
    speed: float,
    points: uint
}


fun draw(p1: &Player, p2: &Player, ball: &Ball, ctx: &win.Window){
    ball.draw()
    p0.draw()
    p1.draw()
    // kdybych nebyl liny tak bych ted vykreslil skore atd..
}

fun main(){
    let ctx = win.init()
    ctx.title("myGame")
    let players = [Player(Sides.Left), Player(Sides.Right)]
    let ball = Ball(1f)
    let running = true
    let gameRunning = true
    while running {
        for e in ctx.get_events() {
            switch e.kind {
            win.EventType.Close {
                running = false
            },
            win.EventType.KeyDown {
                if e.key == win.Keys.S {
                    players[0].move(1f)
                }
                else if e.key == win.Keys.W {
                    players[0].move(-1f)
                }
                else if e.key == win.Keys.ArrowDown {
                    players[1].move(1f)
                }
                else if e.key == win.Keys.ArrowUp {
                    players[1].move(-1f)
                }
            }
            }
        }
        // Game logic
        if gameRunning {
            draw(&players[0], &players[1], &ball, &ctx)
        }
        if players[0].points == 10 || players[1].points == 10 {
            gameRunning = false
        }
    }
}
