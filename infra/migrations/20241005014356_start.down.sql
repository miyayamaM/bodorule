-- トリガーと関数を削除する
DROP TRIGGER IF EXISTS trigger_update_record_updated_at ON boardgames;
DROP FUNCTION IF EXISTS update_record_updated_at;

-- boardgames テーブルを削除する
DROP TABLE IF EXISTS boardgames;
