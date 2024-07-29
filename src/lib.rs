/// Term Definitions for ISO 13400
pub mod definitions {
    /// Network with ECUs exchanging data frames
    pub struct ControllerAreaNetwork;

    /// Host that implements the DoIP protocol
    pub struct DoIPEntity;

    /// Host inside the vehicle which implements the DoIP protocol and thereby provides access to itself and the ECUs of its connected vehicle subnetworks
    pub struct DoIPGateway;

    /// Host inside the vehicle which implements the DoIP protocol to provide access to itself but does not route DoIP protocol data to the vehicle subnetworks
    pub struct DoIPNode;

    /// Off-vehicle device which is used to programme vehicle subsystem ECUs with changed software; a subset of external test equipment
    pub struct ExternalProgrammingEquipment;

    /// Off-vehicle device which is used to obtain information from vehicle subsystem during the act of performing manufacturing, maintenance, diagnostics, and repair.
    pub struct ExternalTestEquipment;

    /// Protocol for packet-switched end-to-end data communication over various transport media
    pub struct InternetProtocol;

    /// Process of modifying network addresses in IP datagram headers while routing.
    pub struct NetworkAddressTranslation;

    /// Transport protocol for connection-oriented data communication via an IP network
    pub struct TransportControlProtocol;

    /// Transport protocol for connectionless data communication via an IP network'
    pub struct UserDatagramProtocol;

    /// Abstract vehicle internal power supply state, which affects the diagnostic capabilities of all servers on the in-vehicle networks and which identifies the state of all servers of all gateway sub-networks that allow diagnostic communication.
    pub struct DiagnosticPowerMode;

    /// Host inside the vehicle, where an Ethernet activation line is terminated and where the link from the first node/host in the external network is terminated.
    pub struct DoIPEdgeNode;

    /// Certificate issued by an intermediate CA to the CoIP entity presented during the TLS handshake to the client DoIP entity to verify the authenticity of this DoIP entity.
    pub struct DoIPEntityCertificate;

    /// Node connected to the IP-based network.
    pub struct Host;

    /// Authority, which issues subordinal certificates to another intermediate CA or DoIP entities.
    pub struct IntermediateCertificateAuthority;

    /// Certificate either stored in the client DoIP entity or is presented during authentication together with the end node certificate to complete the chain of trust.
    pub struct IntermediateCertificate;

    /// Address outside the reserved range for client(s) DoIP entity.
    pub struct InvalidSourceAddress;

    /// Address identifying a diagnostic application layer entity.
    pub struct LogicalAddress;

    /// Device connected to the IP-based network (e.g. Ethernet) and which communicates using Internet protocol but does not implement the DoIP protocol.
    pub struct NetworkNode;

    /// Authority, which acts as the root of trust.
    pub struct RootCertificateAuthority;

    /// Certificate created by the root certificate authority and used as the trust anchor.
    pub struct RootCertificate;

    /// Unique identification, as defined in IETF RFC 147, to or from which information is transmitted in the network.
    pub struct Socket;

    /// Address not listed in the connection table entry.
    pub struct UnknownSourceAddress;

    /// Network not directly connected to the IP-based network.
    pub struct VehicleSubNetwork;
}

/// Abbreviated Terms for ISO 13400
pub mod abbreviated_terms {
    /// Application Layer
    pub struct AL;

    /// Alternative
    pub struct Alt;

    /// Application
    pub struct APP;

    /// Address resolution protocol
    pub struct ARP;

    /// American standard code for information interchange
    pub struct ASCII;

    /// Auto-MDI(XAutomatic medium-dependent interface crossover
    pub struct AutoMDIX;

    /// Certificate Authority
    pub struct CA;

    /// Controller Area Network
    pub struct CAN;

    /// Consecutive Frame
    pub struct CF;

    /// Dynamic Host Control Protocol
    pub struct DHCP;

    /// Data Link Layer
    pub struct DLL;

    /// Domain Name System
    pub struct DNS;

    /// Diagnostic Communication over Internet Protocol
    pub struct DoIP;

    /// Electronic Control Unit
    pub struct ECU;

    /// Entity Identification
    pub struct EID;

    /// First Frame
    pub struct FF;

    /// Failure Mode Indicator
    pub struct FMI;

    /// Group Identification
    pub struct GID;

    /// Graphical User Interface
    pub struct GUI;

    /// Gateway
    pub struct GW;

    /// Internet Assigned Numbers Authority
    pub struct IANA;

    /// Internet Control Message Protocol
    pub struct ICMP;

    /// IETF Internet Engineering Task Force Request for Comments
    pub struct IETFRFC;

    /// Internet Protocol
    pub struct IP;

    /// Internet Protocol Version 4
    pub struct IPv4;

    /// Internet Protocol Version 6
    pub struct IPv6;

    /// Media Access Control
    pub struct MAC;

    /// Message Sequence Chart
    pub struct MSC;

    /// Maximum Transport Unit
    pub struct MTU;

    /// Network Address Translation
    pub struct NAT;

    /// Neighbour Discovery Protocol
    pub struct NDP;

    /// Network Layer
    pub struct NL;

    /// Open Systems Interconnection
    pub struct OSI;

    /// Personal Computer
    pub struct PC;

    /// Public Key Infrastructure
    pub struct PKI;

    /// Source Address
    pub struct SA;

    /// Service Access Point
    pub struct SAP;

    /// Service Data Unit
    pub struct SDU;

    /// Single Frame
    pub struct SF;

    /// Suspect Parameter Number
    pub struct SPN;

    /// Service Primitive Parameter
    pub struct SPP;

    /// Target Address
    pub struct TA;

    /// Transmission Control Protocol
    pub struct TCP;

    /// Transport Layer
    pub struct TL;

    /// Transport Layer Security
    pub struct TLS;

    /// User Datagram Protocol
    pub struct UDP;

    /// Vehicle Identification Number
    pub struct VIN;

    /// Vehicle Manufacturer
    pub struct VM;

    /// Wireless Local Area Network
    pub struct WLAN;

    /// Exclusive Or;
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

/// Vehicle Identification for ISO 13400
pub mod vehicle_identification {
    /// Vehicle identification parameter values
    pub enum VehicleIdParamValues {
        /// `[u8; 17]`
        VIN,

        /// `[u8; 2]`
        LogicalAddress,

        /// `[u8; 6]`
        EID,

        /// `[u8; 6]`
        GID,
    }

    /// Payload type vehicle identification request message - No message parameters
    pub struct VehicleIdReqMsg;

    ///Payload type vehicle identification request message with EID
    pub struct VehicleIdReqMsgEID {
        /// **Description**: This is the DoIP entity's unique ID (e.g. network interface's MAC address) that shall respond to the vehicle identification request message.
        ///
        /// **Values**: If MAC address is used, it shall be in accordance with IEEE EUI-48.
        ///
        /// **Support Condition**: Mandatory.
        pub EID: [u8; 6],
    }

    /// Payload type vehicle identification request message with VIN
    pub struct VehicleIdReqMsgVIN {
        /// **Description**: This is the vehicle's identification number as specified in ISO 3779. This parameter is only present if the client DoIP entity intends to identify the DoIP entities of an individual vehicle, the VIN of which is known to the client DoIP entity.
        ///
        /// **Values**: ASCII.
        ///
        /// **Support Condition**: Mandatory.
        pub VIN: [u8; 17],
    }

    /// Payload type vehicle announcement/identification response message
    pub struct VehicleAnnouncementResMsg {
        /// **Description**: This is the vehicle's VIN as specified in ISO 3779. If the VIN is not configured at the time of transmission of this message, this should be indicated using the invalidity value. In this case the GID is used to associate DoIP nodes with a certain vehicle.
        ///
        /// **Values**: TODO
        ///
        /// **Support Condition**: Mandatory.
        pub VIN: [u8; 17],

        /// **Description**: This is the logical address that is assigned to the responding DoIP entity. The logical address can be used, for example, to address diagnostic requests directly to the DoIP entity.
        ///
        /// **Values**: TODO
        ///
        /// **Support Condition**: Mandatory.
        pub LogicalAddress: [u8; 2],

        /// **Description**: This is a unique identification of the DoIP entities in order to separate their responses even before the VIN is programmed to or recognised by the DoIP devices (e.g. during the vehicle assembly process). It is recommended that the MAC address information of the DoIP entity's network interface be used (one of the interfaces if multiple network interfaces are implemented).
        ///
        /// **Values**: If MAC address is used, it shall be in accordance with IEEE EUI-48.
        ///
        /// **Support Condition**: Mandatory.
        pub EID: [u8; 6],

        /// **Description**: This is a unique identification of the DoIP entities within the same vehicle in teh case that a VIN is not configured for that vehicle. The VIN/GID synchronisation process between DoIP nodes of a vehicle is defined. If the GID is not available at the time of transmission of this message, this shall be indicated using the specific invalidity value.
        ///
        /// **Values**: TODO
        ///
        /// **Support Condition**: Mandatory.
        pub GID: [u8; 6],

        /// **Description**: This is the additional information to notify the client DoIP entity that there are either DoIP entities with no initial connectivity or that a centralised security approach is used.
        ///
        /// **Values**: TODO
        ///
        /// **Support Condition**: Mandatory.
        pub FurtherActionRequires: [u8; 1],

        /// **Description**: This is the additional information to notify the client DoIP entity that all DoIP entities have synchronised their information about the VIN or GID of the vehicle.
        ///
        /// **Values**: TODO
        ///
        /// **Support Condition**: Optional.
        pub VINGIDSyncStatus: [u8; 1],
    }
}
