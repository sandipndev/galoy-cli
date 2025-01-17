use graphql_client::GraphQLQuery;

use rust_decimal::Decimal;

type Username = String;
type WalletId = String;
type SatAmount = Decimal;
type Memo = String;
type Phone = String;
type AuthToken = String;
type OneTimeAuthCode = String;
type SignedAmount = Decimal;

// queries

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/graphql/schema.graphql",
    query_path = "src/client/graphql/queries/default_wallet.graphql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct QueryDefaultWallet;
pub use self::query_default_wallet::QueryDefaultWalletAccountDefaultWallet;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/graphql/schema.graphql",
    query_path = "src/client/graphql/queries/query_globals.graphql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct QueryGlobals;
pub use self::query_globals::QueryGlobalsGlobals;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/graphql/schema.graphql",
    query_path = "src/client/graphql/queries/me.graphql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct QueryMe;
pub use self::query_me::QueryMeMe;

// mutations

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/graphql/schema.graphql",
    query_path = "src/client/graphql/mutations/intraledger_send.graphql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct IntraLedgerPaymentSend;
pub use self::intra_ledger_payment_send::IntraLedgerPaymentSendInput;
pub use self::intra_ledger_payment_send::PaymentSendResult;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/graphql/schema.graphql",
    query_path = "src/client/graphql/mutations/user_login.graphql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct UserLogin;
pub use self::user_login::UserLoginInput;
pub use self::user_login::UserLoginUserLogin;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/graphql/schema.graphql",
    query_path = "src/client/graphql/mutations/request_auth_code.graphql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct UserRequestAuthCode;
pub use self::user_request_auth_code::UserRequestAuthCodeInput;
pub use self::user_request_auth_code::UserRequestAuthCodeUserRequestAuthCode;
