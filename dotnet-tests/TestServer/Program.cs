using Common;
using Opc.Ua;
using Opc.Ua.Configuration;

namespace TestServer;

internal sealed class Program
{
    private static async Task<int> Main(string[] args)
    {
        var configPath = args[0];
        var app = new ApplicationInstance
        {
            ConfigSectionName = "TestServer"
        };
        using var source = new CancellationTokenSource();
        TestServer server;
        ApplicationConfiguration cfg;
        try
        {
            cfg = await app.LoadApplicationConfiguration(configPath, true);
            await app.CheckApplicationInstanceCertificate(false, 0);
            server = new TestServer();
            await app.Start(server);
        }
        catch (ServiceResultException e)
        {
            Comms.Send(new ErrorMessage
            {
                Message = $"Fatal error: {e}. {e.InnerResult?.AdditionalInfo}"
            });
            return 1;
        }
        catch (Exception e)
        {
            Comms.Send(new ErrorMessage
            {
                Message = $"Fatal error: {e}"
            });
            return 1;
        }

        Comms.Send(new ReadyMessage());

        while (!source.Token.IsCancellationRequested)
        {
            await foreach (var message in Comms.ListenToInput(source.Token))
            {
                if (message is ShutdownMessage)
                {
                    source.Cancel();
                }
            }
        }


        return 0;
    }
}
