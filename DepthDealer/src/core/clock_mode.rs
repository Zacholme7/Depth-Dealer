/// Represents the modes of operations for the market maker
/// REALTIME: realtime data and execution
/// BACKTEST: backtesting a strategy

#[derive(Debug)]
pub enum ClockMode {
    REALTIME,
    BACKTEST,
}