### Process steps
1. take `Identity::IdentityOf`, `Identity::Registrars`, `Identity::SuperOf` and `Identity::SuperOf`.
2. update identities's deposit and judgement decimal.
3. update registrars fee decimal.
4. analyze `Identity::SuperOf` and `Identity::SubsOf` and update identity's reserved balance.
5. set `AccountMigration::IdentityOf` and`AccountMigration::Registrars`.