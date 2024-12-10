using System.Text.Json;

namespace Common;

public interface IInMessage;

public class ShutdownMessage : IInMessage
{
}

public enum InMessageType
{
    Shutdown,
}

class InMessageConverter : TaggedUnionConverter<IInMessage, InMessageType>
{
    protected override string TagName => "type";

    protected override IInMessage? FromEnum(JsonDocument document, JsonSerializerOptions options, InMessageType type)
    {
        return type switch
        {
            InMessageType.Shutdown => document.Deserialize<ShutdownMessage>(options),
            _ => throw new JsonException("Unknown type variant")
        };
    }

    protected override InMessageType? ParseEnum(string value)
    {
        return value switch
        {
            "shutdown" => InMessageType.Shutdown,
            _ => null
        };
    }
}