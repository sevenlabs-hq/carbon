{% raw %}use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use solana_sdk::pubkey::Pubkey;

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "{{ account.discriminator }}")]
pub struct {{ account.struct_name }} {
    {% for field in account.fields %}
    pub {{ field.name }}: {{ field.rust_type }},
    {% endfor %}
}
{% endraw %}
