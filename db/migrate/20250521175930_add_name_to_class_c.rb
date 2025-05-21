class AddNameToClassC < ActiveRecord::Migration[8.0]
  def change
    add_column :class_cs, :name, :string
  end
end
