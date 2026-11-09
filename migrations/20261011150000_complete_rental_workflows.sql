ALTER TABLE users
    ADD COLUMN IF NOT EXISTS phone VARCHAR(50);

ALTER TABLE leads
    ADD COLUMN IF NOT EXISTS listing_id UUID REFERENCES listings (id) ON DELETE SET NULL,
    ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

ALTER TABLE tenants
    ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

ALTER TABLE offers
    ADD COLUMN IF NOT EXISTS lead_id UUID REFERENCES leads (id) ON DELETE SET NULL,
    ADD COLUMN IF NOT EXISTS contact_name VARCHAR(255),
    ADD COLUMN IF NOT EXISTS contact_email VARCHAR(255),
    ADD COLUMN IF NOT EXISTS contact_phone VARCHAR(50),
    ADD COLUMN IF NOT EXISTS message TEXT,
    ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

ALTER TABLE viewings
    ADD COLUMN IF NOT EXISTS lead_id UUID REFERENCES leads (id) ON DELETE SET NULL,
    ADD COLUMN IF NOT EXISTS contact_name VARCHAR(255),
    ADD COLUMN IF NOT EXISTS contact_email VARCHAR(255),
    ADD COLUMN IF NOT EXISTS contact_phone VARCHAR(50),
    ADD COLUMN IF NOT EXISTS status VARCHAR(50) NOT NULL DEFAULT 'requested',
    ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

ALTER TABLE transactions
    ADD COLUMN IF NOT EXISTS offer_id UUID REFERENCES offers (id) ON DELETE SET NULL,
    ADD COLUMN IF NOT EXISTS tenant_id UUID REFERENCES tenants (id) ON DELETE SET NULL,
    ADD COLUMN IF NOT EXISTS completed_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

CREATE INDEX IF NOT EXISTS idx_leads_listing_id ON leads (listing_id);
CREATE INDEX IF NOT EXISTS idx_offers_lead_id ON offers (lead_id);
CREATE INDEX IF NOT EXISTS idx_viewings_listing_scheduled_at
    ON viewings (listing_id, scheduled_at);
CREATE INDEX IF NOT EXISTS idx_transactions_offer_id ON transactions (offer_id);

CREATE UNIQUE INDEX IF NOT EXISTS idx_transactions_offer_id_unique
    ON transactions (offer_id)
    WHERE offer_id IS NOT NULL;

DROP TRIGGER IF EXISTS set_leads_updated_at ON leads;
CREATE TRIGGER set_leads_updated_at
BEFORE UPDATE ON leads
FOR EACH ROW EXECUTE FUNCTION set_updated_at();

DROP TRIGGER IF EXISTS set_tenants_updated_at ON tenants;
CREATE TRIGGER set_tenants_updated_at
BEFORE UPDATE ON tenants
FOR EACH ROW EXECUTE FUNCTION set_updated_at();

DROP TRIGGER IF EXISTS set_offers_updated_at ON offers;
CREATE TRIGGER set_offers_updated_at
BEFORE UPDATE ON offers
FOR EACH ROW EXECUTE FUNCTION set_updated_at();

DROP TRIGGER IF EXISTS set_viewings_updated_at ON viewings;
CREATE TRIGGER set_viewings_updated_at
BEFORE UPDATE ON viewings
FOR EACH ROW EXECUTE FUNCTION set_updated_at();

DROP TRIGGER IF EXISTS set_transactions_updated_at ON transactions;
CREATE TRIGGER set_transactions_updated_at
BEFORE UPDATE ON transactions
FOR EACH ROW EXECUTE FUNCTION set_updated_at();
