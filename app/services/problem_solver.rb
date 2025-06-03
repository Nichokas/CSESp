module ProblemSolver
  def self.interior(text)
    text.downcase
  end
  def self.velocidad(text)
    text.to_s.gsub(" ", "...")
  end
    def self.caras(text)
    text.to_s.gsub(":)", "🙂").gsub(":(", "🙁")
  end
end
