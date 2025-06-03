module ConversationProblemHelper
  def self.find(sname)
    case sname
    when "interior"
      true
    when "velocidad"
      true
    when "caras"
      true
    else
      false
    end
  end
  def self.g_test(sname)
    case sname
    when "interior"
      simbolset = [".",",",";",":","!","?","-","_","+","*"]
      "#{rand 99999999}#{SecureRandom.alphanumeric(52)}#{simbolset[rand(0..simbolset.length)]}"
    when "velocidad"
      "#{rand 99999999} #{SecureRandom.alphanumeric(52)}"
    when "caras"
      faces = [":)", ":("]
      base_text = "#{SecureRandom.alphanumeric(10)}"
      (2 + rand(3)).times do
        insertion_point = rand(base_text.length)
        base_text = base_text.insert(insertion_point, faces[rand(2)])
      end
      base_text
    else
      false
    end
  end
end
