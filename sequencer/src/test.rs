
trait TransactionType {}

pub struct TransactionStore {
    mempool :  VecDeque<TransactionType>
}