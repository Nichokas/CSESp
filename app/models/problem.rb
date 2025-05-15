class Problem < ApplicationRecord
  validates :name, presence: true
  validates :steps, presence: true
  validates :files, presence: true
end
