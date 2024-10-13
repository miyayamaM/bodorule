-- トリガーと関数を削除する
DROP TRIGGER IF EXISTS trigger_update_record_updated_at ON board_games;
DROP FUNCTION IF EXISTS update_record_updated_at;

-- board_games テーブルを削除する
DROP TABLE IF EXISTS board_games;
