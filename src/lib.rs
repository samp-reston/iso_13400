// Definitions for ISO 13400
pub mod definitions {
    /// Controller Area Network (CAN) // Network with ECUs exchanging data frames
    pub struct ControllerAreaNetwork;

    /// DoIP entity // host that implements the DoIP protocol
    pub struct DoIPEntity;

    /// DoIP gateway // Host inside the vehicle which implements the DoIP protocol and thereby provides access to itself and the ECUs of its connected vehicle subnetworks
    pub struct DoIPGateway;

    /// DoIP Node // Host inside the vehicle which implements the DoIP protocol to provide access to itself but does not route DoIP protocol data to the vehicle subnetworks
    pub struct DoIPNode;

    /// External Programming Equipment // Off-vehicle device which is used to programme vehicle subsystem ECUs with changed software; a subset of external test equipment
    pub struct ExternalProgrammingEquipment;

    /// External Test Equipment // Off-vehicle device which is used to obtain information from vehicle subsystem during the act of performing manufacturing, maintenance, diagnostics, and repair.
    pub struct ExternalTestEquipment;

    /// Internet Protocol (IP) // Protocol for packet-switched end-to-end data communication over various transport media
    pub struct InternetProtocol;

    /// Network Address Translation (NAT) // Process of modifying network addresses in IP datagram headers while routing.
    pub struct NetworkAddressTranslation;

    /// Transport Control Protocol (TCP) // Transport protocol for connection-oriented data communication via an IP network
    pub struct TransportControlProtocol;

    /// User Datagram Protocol (UDP) // Transport protocol for connectionless data communication via an IP network'
    pub struct UserDatagramProtocol;

    /// Diagnostic Power Mode // Abstract vehicle internal power supply state, which affects the diagnostic capabilities of all servers on the in-vehicle networks and which identifies the state of all servers of all gateway sub-networks that allow diagnostic communication.
    pub struct DiagnosticPowerMode;

    /// DoIP Edge Node // Host inside the vehicle, where an Ethernet activation line is terminated and where the link from the first node/host in the external network is terminated.
    pub struct DoIPEdgeNode;

    /// DoIP Entity Certificate // Certificate issued by an intermediate CA to the CoIP entity presented during the TLS handshake to the client DoIP entity to verify the authenticity of this DoIP entity.
    pub struct DoIPEntityCertificate;

    /// Host // Node connected to the IP-based network.
    pub struct Host;

    /// Intermediate Certificate Authority (Intermediate CA) // Authority, which issues subordinal certificates to another intermediate CA or DoIP entities.
    pub struct IntermediateCertificateAuthority;

    /// Intermediate Certificate // Certificate either stored in the client DoIP entity or is presented during authentication together with the end node certificate to complete the chain of trust.
    pub struct IntermediateCertificate;

    /// Invalid Source Address // Address outside the reserved range for client(s) DoIP entity.
    pub struct InvalidSourceAddress;

    /// Logical Address // Address identifying a diagnostic application layer entity.
    pub struct LogicalAddress;

    /// Network Node // Device connected to the IP-based network (e.g. Ethernet) and which communicates using Internet protocol but does not implement the DoIP protocol.
    pub struct NetworkNode;

    /// Root Certificate Authority // Authority, which acts as the root of trust.
    pub struct RootCertificateAuthority;

    /// Root Certificate // Certificate created by the root certificate authority and used as the trust anchor.
    pub struct RootCertificate;

    /// Socket // Unique identification, as defined in IETF RFC 147, to or from which information is transmitted in the network.
    pub struct Socket;

    /// Unknown Source Address // Address not listed in the connection table entry.
    pub struct UnknownSourceAddress;

    /// Vehicle Sub-Network // Network not directly connected to the IP-based network.
    pub struct VehicleSubNetwork;
}

// Abbreviated Terms for ISO 13400
pub mod abbreviated_terms {
    /// AL // Application Layer
    pub struct AL;

    /// Alt // Alternative
    pub struct Alt;

    /// APP // Application
    pub struct APP;

    /// ARP // Address resolution protocol
    pub struct ARP;

    /// ASCII // American standard code for information interchange
    pub struct ASCII;

    /// Auto-MDI(X) // Automatic medium-dependent interface crossover
    pub struct AutoMDIX;

    /// CA // Certificate Authority
    pub struct CA;

    /// CAN // Controller Area Network
    pub struct CAN;

    /// CF // Consecutive Frame
    pub struct CF;

    /// DHCP // Dynamic Host Control Protocol
    pub struct DHCP;

    /// DLL // Data Link Layer
    pub struct DLL;

    /// DNS // Domain Name System
    pub struct DNS;

    /// DoIP // Diagnostic Communication over Internet Protocol
    pub struct DoIP;

    /// ECU // Electronic Control Unit
    pub struct ECU;

    /// EID // Entity Identification
    pub struct EID;

    /// FF // First Frame
    pub struct FF;

    /// FMI // Failure Mode Indicator
    pub struct FMI;

    /// GID // Group Identification
    pub struct GID;

    /// GUI // Graphical User Interface
    pub struct GUI;

    /// GW // Gateway
    pub struct GW;

    /// IANA // Internet Assigned Numbers Authority
    pub struct IANA;

    /// ICMP // Internet Control Message Protocol
    pub struct ICMP;

    /// IETF RFC // Internet Engineering Task Force Request for Comments
    pub struct IETFRFC;

    /// IP // Internet Protocol
    pub struct IP;

    /// IPv4 // Internet Protocol Version 4
    pub struct IPv4;

    /// IPv6 // Internet Protocol Version 6
    pub struct IPv6;

    /// MAC // Media Access Control
    pub struct MAC;

    /// MSC // Message Sequence Chart
    pub struct MSC;

    /// MTU // Maximum Transport Unit
    pub struct MTU;

    /// NAT // Network Address Translation
    pub struct NAT;

    /// NDP // Neighbour Discovery Protocol
    pub struct NDP;

    /// NL // Network Layer
    pub struct NL;

    /// OSI // Open Systems Interconnection
    pub struct OSI;

    /// PC // Personal Computer
    pub struct PC;

    /// PKI // Public Key Infrastructure
    pub struct PKI;

    /// SA // Source Address
    pub struct SA;

    /// SAP // Service Access Point
    pub struct SAP;

    /// SDU // Service Data Unit
    pub struct SDU;

    /// SF // Single Frame
    pub struct SF;

    /// SPN // Suspect Parameter Number
    pub struct SPN;

    /// SPP // Service Primitive Parameter
    pub struct SPP;

    /// TA // Target Address
    pub struct TA;

    /// TCP // Transmission Control Protocol
    pub struct TCP;

    /// TL // Transport Layer
    pub struct TL;

    /// TLS // Transport Layer Security
    pub struct TLS;

    /// UDP // User Datagram Protocol
    pub struct UDP;

    /// VIN // Vehicle Identification Number
    pub struct VIN;

    /// VM // Vehicle Manufacturer
    pub struct VM;

    /// WLAN // Wireless Local Area Network
    pub struct WLAN;

    /// XOR // Exclusive Or;
    pub struct XOR;

    pub enum Symbols {
        /// `<d>` Payload length, given in bytes.
        D,
        /// `<m>` Number of concurrent DoIP TCP sessions that the client DoIP entity is required to support in order to connect to one or more DoIP entities.
        M,
        /// `<n>` Number of concurrent DoIP TCP sessions that the DoIP entity needs to support in order to accept 1 to N concurrent connections to one or more items of the client DoIP entity.
        N,
        /// `<u>` Number of individual servers in a vehicle sub-network.
        U,
        /// `<v>` Number of individual servers in a vehicle sub-network.
        V,
        /// `<w>` Number of individual DoIP gateways in a vehicle network.
        W,
        /// `<x>` Number of individual in-vehicle network nodes.
        X,
        /// `<y>` Number of individual vehicle DoIP nodes in a vehicle network.
        Y,
        /// `<z>` Number of individual vehicle external network nodes.
        Z,
    }
}
