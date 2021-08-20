use ic_cdk::export::candid;

#[ic_cdk_macros::query]
fn print() -> candid::Nat {
    ic_cdk::print("Hello World from DFINITY!");
    let hello = candid::Nat::from(10);
    // unsafe{
        hello.clone()
    // }
}