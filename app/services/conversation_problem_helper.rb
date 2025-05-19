module ConversationProblemHelper
  def self.find(sname)
    case sname
    when "interior"
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
    else
      false
    end
  end
end