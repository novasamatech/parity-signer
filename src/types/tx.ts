export type TxParticipant = {
	address: string;
	networkKey: string;
};

export interface Tx {
	sender: TxParticipant;
	recipient: TxParticipant;
	hash: string;
}
