use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    //single value transmitted
    // thread::spawn(move || {
    //     tx.send("77997899").unwrap();
    // });

    //multi value transmitted
    // thread::spawn(move || {
    //     let msgs = vec!["data 1: 98989889", "data 2: 787878788", "data 3: 454545454"];
    //
    //     for msg in msgs {
    //         tx.send(msg).unwrap();
    //     }
    // });

    //multi producers - multi value transmitted
    let tx1 = tx.clone();

    thread::spawn(move || {
        let msgs = vec![
            "data tx_1: 98989889",
            "data tx_2: 787878788",
            "data tx_3: 454545454",
        ];

        for msg in msgs {
            tx.send(msg).unwrap();
        }
    });

    thread::spawn(move || {
        let msgs = vec![
            "data tx1_1: trrtr",
            "data tx1_2: hmjhmjmjm",
            "data tx1_3: rereerre",
        ];

        for msg in msgs {
            tx1.send(msg).unwrap();
        }
    });

    //receiver single value
    //let receiver = rx.recv().unwrap();
    // dbg!(receiver);

    //receiver multi value
    for rec in rx {
        dbg!(rec);
    }
}
