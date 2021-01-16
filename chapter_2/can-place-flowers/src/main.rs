fn main() {
    let flowerbed = vec![1,0,0,0,1];
    assert!(can_place_flowers(flowerbed, 1));
    let flowerbed = vec![0,0,1,0,1];
    assert!(can_place_flowers(flowerbed, 1));
    let flowerbed = vec![0,1,1,0,0];
    assert!(can_place_flowers(flowerbed, 1));
    let flowerbed = vec![1,0,0,0,0,1];
    assert!(!can_place_flowers(flowerbed, 2));

}

pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut c = n;
    let mut flowerbed = flowerbed;
    for i in 0..flowerbed.len() {
        if c <=0 {
            break;
        }
        if flowerbed[i] == 1 || (i>= 1 && flowerbed[i-1] == 1) {
            continue;
        }
        if i == flowerbed.len() - 1 || flowerbed[i+1] == 0 {
            flowerbed[i] = 1;
            c-=1;
        }
    }
    c <= 0
}
