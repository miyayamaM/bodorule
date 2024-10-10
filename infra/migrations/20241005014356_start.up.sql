-- record_updated_at を更新する関数
CREATE OR REPLACE FUNCTION update_record_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.record_updated_at = CURRENT_TIMESTAMP(3);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- boardgames テーブルの定義
CREATE TABLE IF NOT EXISTS boardgames (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    thumbnail_url VARCHAR(1000),
    record_created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    record_updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

CREATE TRIGGER trigger_update_record_updated_at
BEFORE UPDATE ON boardgames
FOR EACH ROW
EXECUTE FUNCTION update_record_updated_at();
