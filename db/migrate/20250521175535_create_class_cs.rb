class CreateClassCs < ActiveRecord::Migration[8.0]
  def change
    create_table :class_cs, id: :uuid do |t|

      t.timestamps
    end
  end
end
