class Problem < ApplicationRecord
  validates :name, presence: true
  validates :problem_description, presence: true
  validates :demo, presence: true
end
