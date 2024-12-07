-- Add migration script here
ALTER TABLE transactions
MODIFY status ENUM(
        'requires_payment_method',
        'requires_confirmation',
        'requires_action',
        'processing',
        'requires_capture',
        'canceled',
        'succeeded',
        'refunded'
    ) DEFAULT 'requires_payment_method' NOT NULL;