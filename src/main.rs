use game_of_life::Board;
fn main (){
    let mut a = vec![0, 0, 1];
    let mut b = vec![1, 0, 1];
    let mut c = vec![1, 1, 1];


    let mut a_sum = vec![0,0,0];
    let mut b_sum = vec![0,0,0];
    let mut c_sum = vec![0,0,0];


    let mut d_sum = vec![& mut a_sum, &mut b_sum, &mut c_sum];
    let mut d = vec![&mut a, &mut b, &mut c];
    let mut test = Board {
        size: 3,
        initial_grid: & mut d,
        sum_grid: &mut d_sum,
    };
    test.start();

}
