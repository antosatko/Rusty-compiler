import "std.time" as time;
import "std.window" as win;

const S_WIDTH: uint = 650;
const S_HEIGHT: uint = 400;


struct Ball {
    x: float,
    y: float,
    r: float,
    xs: float,
    ys: float
}

impl Ball {
    fun constructor(direction: float){
        self.x = S_WIDTH as float / 2f;
        self.y = S_HEIGHT as float / 2f;
        self.r = 5f;
        self.xs = direction;
        self.ys = 0f;
    }
    fun move(&self) {
        self.x += self.xs;
        self.y += self.ys;
    }
    fun draw(&self) {
        // neco
    }
    fun draw(&self, ctx: win.Window) {
        
    }
}

enum Sides {
    Left = 50,
    Right = 600
}

struct Player {
    side: Sides,
    y: float,
    w: float,
    h: float,
    speed: float,
    points: uint
}

impl Player {
    fun constructor(side: Sides){
        self.y = 0f;
        self.w = 20f;
        self.h = 100f;
        self.speed = 1f;
        self.points = 0;
        self.side = side;
    }
    fun move(&self, direction: float) {
        self.y += self.speed * direction;
        if self.y < 0 {
            self.y = 0;
        }
        else if self.y > S_HEIGHT - self.h {
            self.y = S_HEIGHT - self.h;
        }
    }
    fun collision(&self, ball: Ball) {
        if self.side as int < ball.x + ball.r / 2 && self.side as int + self.w > ball.x - ball.r / 2 &&
            self.y < ball.y + ball.r / 2 && self.y + self.h > ball.y - ball.r / 2 
        {
            // collision detected
            // too lazy to do something rn
            ball.xs *= -1f;
        }
    }
}

fun main(){
    let ctx = win.init();
    ctx.title("myGame");
    let players = [Player(Sides.Left), Player(Sides.Right)];
    let ball = Ball(1f);
    let running = true;
    while running {
        for e in ctx.get_events() {
            switch e.type {
            win.EventType.Close {
                break;
            },
            win.EventType.KeyDown {
                if e.key == win.Keys.S {
                    players[0].move(1f)
                }
                if e.key == win.Keys.W {
                    players[0].move(-1f)
                }
                if e.key == win.Keys.ArrowDown {
                    players[1].move(1f)
                }
                if e.key == win.Keys.ArrowUp {
                    players[1].move(-1f)
                }
            }
            }
        }
        // Game logic
        ball.move();
        players[0].collision(&ball);
        players[1].collision(&ball);
    }
}
