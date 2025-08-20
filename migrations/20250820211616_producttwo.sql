-- Add migration script here
-- Add migration script here
-- Status table
CREATE TABLE status (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE
);

-- Insert default statuses
INSERT INTO status (name) VALUES ('PUBLISHED'), ('UNPUBLISHED')
ON CONFLICT (name) DO NOTHING;

-- Product table
CREATE TABLE product (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID NOT NULL,
    vendor_id UUID NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    points INT NOT NULL,
    initial_quantity INT,
    quantity_limit BOOLEAN,
    status_id UUID NOT NULL REFERENCES status(id),
    created_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID NOT NULL,
    modified_date TIMESTAMP,
    modified_by UUID,
    deleted_date TIMESTAMP,
    deleted_by UUID
);
