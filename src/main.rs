fn check(goal: usize, n: usize, ads: &Vec<[usize; 2]>) -> bool {
    let mut valid_combinations: usize = 0;
    for i in 0..2_usize.pow(n as u32) {
        let binary = format!("{:0n$b}", i, n = n);
        let mut valid = false;
        for k in 0..ads.len() {
            if binary.chars().nth(ads[k][0]).unwrap() == '0'
                && binary.chars().nth(ads[k][1]).unwrap() == '1'
            {
                valid = true;
                break;
            }
        }
        if !valid {
            valid_combinations += 1;
        }
    }
    valid_combinations == goal
}

fn main() {
    let goal = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let mut ads: Vec<[usize; 2]> = Vec::new();
    for nodes in 2..=9 {
        ads.push([nodes - 2, nodes - 1]);
        let mut new = 1;
        while !check(goal[nodes - 1], nodes, &ads) {
            let foo = ads.len() - 1;
            if ads[foo][0] == 0 {
                // 1 new relation not enough?
                if new >= 2 {
                    ads[foo][0] = nodes - 2;
                    if ads[foo - 1][0] == 0 {
                        // 2 new relations not enough? Stop.
                        break;
                    } else {
                        ads[foo - 1][0] -= 1;
                    }
                } else {
                    ads[foo][0] = nodes - 2;
                    ads.push([nodes - 2, nodes - 1]);
                    new = 2;
                }
            } else {
                ads[foo][0] -= 1;
            }
        }
        println!("{} nodes: {:?}", nodes, ads);
    }
}

/*
@startuml
rectangle "23 combinations" {
rectangle "19 combinations" {
rectangle "17 combinations" {
rectangle "13 combinations" {
rectangle "11 combinations" {
rectangle "7 combinations" {
rectangle "5 combinations" {
rectangle "3 combinations" {
rectangle "2 combinations" {
storage 1
}
storage 2
}
storage 3
}
storage 4
}
storage 5
}
storage 6
}
storage 7
}
storage 8
}
storage 9
}
1 --> 2
1 --> 3
3 --> 4
3 --> 5
5 --> 6
4 --> 6
4 --> 7
5 --> 7
6 --> 8
7 --> 8
6 --> 9
7 --> 9
@enduml
*/
