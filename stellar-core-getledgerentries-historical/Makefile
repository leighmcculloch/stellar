install:
	HOMEBREW_NO_INSTALL_FROM_API=1 \
		brew install \
		--formula \
		--build-from-source \
		--head \
		stellar-core.rb

run:
	stellar-core run

monitor:
	while true; do http :11626/info | jq -r '.info | [.state, .status]'; sleep 1; done

test:
	http -v --form \
		:11726/getledgerentryraw \
		key="$(echo '{"contract_data":{"contract": "CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA","key": "ledger_key_contract_instance","durability": "persistent"}}' | stellar xdr encode --type LedgerKey)" \
		ledgerSeq=$(LEDGER)
