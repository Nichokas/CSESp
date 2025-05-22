module ProblemSolver
  def self.interior(text)
    text.downcase
  end
  def self.velocidad(text)
    text.to_s.gsub(" ", "...")
  end
end